use super::*;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Something went wrong! No clue what though. GLHF!")]
    Unhelpful,
}
pub fn validate_conf(_conf: &WithValidationConfig) -> Result<(), ValidationError> {
    Err(ValidationError::Unhelpful)?;

    Ok(())
}
