// !- Modules

pub(crate) mod error;
pub use error::{BuildConfigError, MergeConfigError};

// #[doc(hidden)]
// pub mod sealed;

#[doc(hidden)]
pub mod __private {
    // expose impl checks for derived
    pub use static_assertions; 
    
    pub use super::error::{ConfigErrorBundled, error_field_list};
}