use crate::{
	flags::Flags,
	mdb::{error::mdb_result, ffi},
	Error, Result,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::{
	any::TypeId,
	collections::hash_map::{Entry, HashMap},
	ffi::CString,
	fs::File,
	io,
	path::{Path, PathBuf},
	ptr,
	sync::{self, Arc, RwLock},
	time::Duration,
};
use synchronoise::SignalEvent;

type OpenedEnv = Lazy<RwLock<HashMap<PathBuf, (Option<Env>, Arc<SignalEvent>)>>>;

static OPENED_ENV: OpenedEnv = Lazy::new(Default::default);

#[cfg(not(windows))]
fn canonicalize_path(path: &Path) -> io::Result<PathBuf> {
	path.canonicalize()
}

#[cfg(windows)]
fn canonicalize_path(path: &Path) -> io::Result<PathBuf> {
	let canonical = path.canonicalize()?;
	let url = url::Url::from_file_path(&canonical)
		.map_err(|_| io::Error::new(io::ErrorKind::Other, "URL passing error"))?;
	url.to_file_path()
		.map_err(|_| io::Error::new(io::ErrorKind::Other, "path canonicalization error"))
}

#[cfg(windows)]
trait OsStrExtLmdb {
	fn as_bytes(&self) -> &[u8];
}

#[cfg(windows)]
impl OsStrExtLmdb for OsStr {
	fn as_bytes(&self) -> &[u8] {
		self.to_str().unwrap().as_bytes()
	}
}

#[cfg(windows)]
fn get_file_fd(file: &File) -> std::os::windows::io::RawHandle {
	use std::os::windows::io::AsRawHandle;
	file.as_raw_handle()
}

#[cfg(unix)]
fn get_file_fd(file: &File) -> std::os::unix::io::RawFd {
	use std::os::unix::io::AsRawFd;
	file.as_raw_fd()
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EnvOpenOptions {
	map_size: Option<usize>,
	max_readers: Option<u32>,
	max_dbs: Option<u32>,
	flags: u32,
}

impl EnvOpenOptions {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			map_size: None,
			max_readers: None,
			max_dbs: None,
			flags: 0,
		}
	}

	pub fn map_size(&mut self, size: usize) -> &mut Self {
		self.map_size = Some(size);

		self
	}

	pub fn max_readers(&mut self, readers: u32) -> &mut Self {
		self.max_readers = Some(readers);

		self
	}

	pub fn max_dbs(&mut self, dbs: u32) -> &mut Self {
		self.max_dbs = Some(dbs);

		self
	}

	pub unsafe fn flags(&mut self, flags: Flags) -> &mut Self {
		self.flags |= flags as u32;

		self
	}

	#[allow(clippy::shadow_unrelated)]
	pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<Env> {
		let path = canonicalize_path(path.as_ref())?;

		let mut lock = OPENED_ENV.write().unwrap();

		match lock.entry(path) {
			Entry::Occupied(entry) => entry.get().0.clone().ok_or(Error::DatabaseClosing),
			Entry::Vacant(entry) => {
				let path = entry.key();
				let path_str = CString::new(path.as_os_str().as_bytes()).unwrap();

				unsafe {
					let mut env = ptr::null_mut();
					mdb_result(ffi::mdb_env_create(&mut env))?;

					if let Some(size) = self.map_size {
						if size % page_size::get() != 0 {
							let msg = format!(
								"map size ({}) must be a multiple of the system page size ({})",
								size,
								page_size::get()
							);
							return Err(Error::Io(io::Error::new(
								io::ErrorKind::InvalidInput,
								msg,
							)));
						}
						mdb_result(ffi::mdb_env_set_mapsize(env, size))?;
					}

					if let Some(readers) = self.max_readers {
						mdb_result(ffi::mdb_env_set_maxreaders(env, readers))?;
					}

					if let Some(dbs) = self.max_dbs {
						mdb_result(ffi::mdb_env_set_maxdbs(env, dbs))?;
					}

					let flags = if cfg!(feature = "sync-read-txn") {
						self.flags | Flags::NoTls as u32
					} else {
						self.flags
					};

					let result =
						mdb_result(ffi::mdb_env_open(env, path_str.as_ptr(), flags, 0o600));

					match result {
						Ok(()) => {
							let signal_handler = Arc::new(SignalEvent::manual(false));
							let inner = EnvInner {
								env,
								dbi_open_mutex: sync::Mutex::default(),
								path: path.clone(),
							};

							let env = Env(Arc::new(inner));
							entry.insert((Some(env.clone()), signal_handler));
							Ok(env)
						}
						Err(e) => {
							ffi::mdb_env_close(env);
							Err(e.into())
						}
					}
				}
			}
		}
	}
}

pub fn env_closing_event<P: AsRef<Path>>(path: P) -> Option<EnvClosingEvent> {
	let lock = OPENED_ENV.read().unwrap();
	lock.get(path.as_ref())
		.map(|(_, se)| EnvClosingEvent(se.clone()))
}

#[derive(Debug, Clone)]
pub struct Env(Arc<EnvInner>);

impl Env {
    pub fn copy_to_path<P: AsRef<Path>>(&self, path: P, option: bool) -> Result<File> {
        let file = File::create(&path)?;
        let fd = get_file_fd(&file);

        unsafe {self.copy_to_fd(fd, option)?;}

        let file = File::open(path)?;

        Ok(file)
    }

    pub unsafe fn copy_to_fd(&self, fd: ffi::mdb_filehandle_t, option: bool) -> Result<()> {
        let flags = if option {
            ffi::MDB_CP_COMPACT
        } else {0};

        mdb_result(ffi::mdb_env_copy2fd(self.0.env, fd, flags))?;

        Ok(())
    }

	#[cfg(feature = "lmdb")]
	pub fn force_sync(&self) -> Result<()> {
		unsafe { mdb_result(ffi::mdb_env_sync(self.0.env, 1))? }

		Ok(())
	}

	#[cfg(feature = "mdbx")]
	pub fn force_sync(&self) -> Result<()> {
		unsafe { mdb_result(ffi::mdb_env_sync(self.0.env))? }

		Ok(())
	}

	#[must_use]
	pub fn path(&self) -> &Path {
		&self.0.path
	}

	#[must_use]
	pub fn prepare_for_closing(self) -> EnvClosingEvent {
		let mut lock = OPENED_ENV.write().unwrap();
		let env = lock.get_mut(&self.0.path);

		match env {
			None => panic!("cannot find the env that we are trying to close"),
			Some((env, signal_event)) => {
				let _env = env.take();
				let signal_event = signal_event.clone();

				drop(lock);

				EnvClosingEvent(signal_event)
			}
		}
	}
}

#[derive(Debug)]
struct EnvInner {
	env: *mut ffi::MDB_env,
	dbi_open_mutex: sync::Mutex<HashMap<u32, (TypeId, TypeId)>>,
	path: PathBuf,
}

unsafe impl Send for EnvInner {}
unsafe impl Sync for EnvInner {}

impl Drop for EnvInner {
	fn drop(&mut self) {
		let mut lock = OPENED_ENV.write().unwrap();

		match lock.remove(&self.path) {
			None => panic!("It seems another env closed this env before"),
			Some((_, signal_event)) => {
				unsafe {
					ffi::mdb_env_close(self.env);
				};

				signal_event.signal();
			}
		}
	}
}

pub struct EnvClosingEvent(Arc<SignalEvent>);

impl EnvClosingEvent {
	pub fn wait(&self) {
		self.0.wait();
	}

	#[must_use]
	pub fn wait_timeout(&self, time: Duration) -> bool {
		self.0.wait_timeout(time)
	}
}
