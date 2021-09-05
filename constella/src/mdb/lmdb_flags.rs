// LMDB flags (see http://www.lmdb.tech/doc/group__mdb__env.html for more details).
#[repr(u32)]
pub enum Flags {
	Fixedmap = lmdb_sys::MDB_FIXEDMAP,
	NoSubDir = lmdb_sys::MDB_NOSUBDIR,
	NoSync = lmdb_sys::MDB_NOSYNC,
	RdOnly = lmdb_sys::MDB_RDONLY,
	NoMetaSync = lmdb_sys::MDB_NOMETASYNC,
	WriteMap = lmdb_sys::MDB_WRITEMAP,
	MapAsync = lmdb_sys::MDB_MAPASYNC,
	NoTls = lmdb_sys::MDB_NOTLS,
	NoLock = lmdb_sys::MDB_NOLOCK,
	NoRdAhead = lmdb_sys::MDB_NORDAHEAD,
	NoMemInit = lmdb_sys::MDB_NOMEMINIT,
}
