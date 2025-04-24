use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    Uuid::parse_str(uuid).map_err(|_e| ValidationError::new("Invalid uuid format"))?;
    Ok(())
}
