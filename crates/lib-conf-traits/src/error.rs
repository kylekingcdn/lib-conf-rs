use std::error::Error; 

/// Implemented for `Config` struct error enums when the `Config` struct (or a nested config) has a fallible call
///
/// Possible scenarios include, but are not limited to:
///
/// - Validation failures
/// - A field was missing when trying to initialize the `Config` struct using only it's `Override` type
/// - A failure occurred in a nested config
///   - There should be a dedicated variant for each field with a nested Config that provides a `ConfigError` type.
///   - These variants should wrap the inner error
pub trait ConfigError: Error {
    /// Builds a list of fields leading to the source error
    ///
    /// # Implementation notes
    ///
    /// Implementors should:
    ///
    /// 1. append `self_field_name` to the provided `chain` list
    /// 2. match against own variants, if variant is nested:
    ///    - recurse on the inner error, passing the updated `chain` along with the name of the matching field
    fn build_field_chain(&self, chain: &mut Vec<String>, self_field_name: Option<&str>);
    
    /// Returns the source error message
    ///
    /// Used to generate the full error including the nested path
    fn origin_message(&self) -> String;

    /// List of field names leading to the source of a nested error.
    ///
    /// Does not include the offending field(s) themselves, just the sequence of fields leading to the
    /// containing struct
    fn source_chain(&self) -> Vec<String> {
        let mut chain = Vec::new();
        self.build_field_chain(&mut chain, None);
        chain
    }
    /// Same as [`source_chain()`](Self::source_chain), but returns the chain as a single dot-delimited string
    fn source_path(&self) -> String {
        self.source_chain().join(".")
    }
}
