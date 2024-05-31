use anyhow::Result;
use indexmap::IndexMap;
use strum::Display;

pub mod prelude;
#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(feature = "vrcdb")]
pub mod vrcdb;
#[cfg(feature = "nekodb")]
pub mod nekodb;

#[derive(Display, Eq, Hash, PartialEq)]
pub enum Type {
    #[cfg(feature = "cache")]
    Cache,
    #[cfg(feature = "sqlite")]
    Sqlite,
    #[cfg(feature = "vrcdb")]
    #[strum(to_string = "Avatar Search")]
    VRCDB,
    #[cfg(feature = "nekodb")]
    #[strum(to_string = "Avatar Search")]
    NEKODB,
}

pub trait Provider {
    /// True: New/Unique | False: Duplicate/Existing
    ///
    /// # Errors
    ///
    /// Will return `Err` if anything errors
    fn check_avatar_id(&self, avatar_id: &str) -> Result<bool>;

    /// True: New/Unique | False: Duplicate/Existing
    ///
    /// # Errors
    ///
    /// Will return `Err` if anything errors
    fn send_avatar_id(&self, avatar_id: &str) -> Result<bool>;
}

pub type Providers = IndexMap<Type, Box<dyn Provider>>;

// https://stackoverflow.com/a/72239266
#[macro_export]
macro_rules! box_db {
    ($x:expr) => {
        Box::new($x) as Box<dyn $crate::provider::Provider>
    };
}
