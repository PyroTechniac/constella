// MDBX flags (see https://github.com/erthink/libmdbx/blob/master/mdbx.h for more details).
#[repr(u32)]
pub enum Flags {
	NoSubDir = mdbx_sys::MDBX_NOSUBDIR,
	RdOnly = mdbx_sys::MDBX_RDONLY,
	NoMetaSync = mdbx_sys::MDBX_NOMETASYNC,
	WriteMap = mdbx_sys::MDBX_WRITEMAP,
	MapAsync = mdbx_sys::MDBX_MAPASYNC,
	NoTls = mdbx_sys::MDBX_NOTLS,
	NoRdAhead = mdbx_sys::MDBX_NORDAHEAD,
	NoMemInit = mdbx_sys::MDBX_NOMEMINIT,
}
