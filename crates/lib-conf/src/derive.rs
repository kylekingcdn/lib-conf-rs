// !- Modules

pub(crate) mod error;

#[doc(hidden)]
pub mod __private {
    // expose impl checks for derived
    pub use static_assertions; 
    
    pub use super::error::error_field_list;
}