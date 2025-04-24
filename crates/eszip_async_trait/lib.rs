use eszip::Module;

pub static POWERBASE_ESZIP_VERSION: &[u8] = b"1.1";

pub static POWERBASE_ESZIP_VERSION_KEY: &str = "---POWERBASE-ESZIP-VERSION-ESZIP---";
pub static VFS_ESZIP_KEY: &str = "---POWERBASE-VFS-DATA-ESZIP---";
pub static SOURCE_CODE_ESZIP_KEY: &str = "---POWERBASE-SOURCE-CODE-ESZIP---";
pub static STATIC_FILES_ESZIP_KEY: &str = "---POWERBASE-STATIC-FILES-ESZIP---";
pub static NPM_RC_SCOPES_KEY: &str = "---POWERBASE-NPM-RC-SCOPES---";

pub trait AsyncEszipDataRead: std::fmt::Debug + Send + Sync {
    fn ensure_module(&self, specifier: &str) -> Option<Module>;
    fn ensure_import_map(&self, specifier: &str) -> Option<Module>;
}
