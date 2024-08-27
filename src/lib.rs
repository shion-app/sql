mod decode;
mod error;
mod plugin;
mod transaction;

pub use error::{Error, Result};
use plugin::Builder;

pub use plugin::{DbInstances, Migration, MigrationKind};

pub fn init() -> Builder {
    Builder::default()
}
