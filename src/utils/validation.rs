use std::borrow::Cow;
use validator::ValidationError;


pub fn validate_username_chars(username: &str) -> Result<(), ValidationError> {
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::new("username_chars")
            .with_message("Username can only contain letters, numbers and underscores".into()));
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let username = username.trim();
    
    if username.is_empty() || username.len() < 4 || username.len() > 30 {
        return Err(ValidationError::new("invalid_length")
            .with_message(Cow::Borrowed("Username must be between 4 and 30 characters")));
    }
    
    if !username.is_ascii() {
        return Err(ValidationError::new("non_ascii")
            .with_message(Cow::Borrowed("Only English characters allowed")));
    }

    let valid = username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
    
    if !valid {
        return Err(ValidationError::new("invalid_username")
            .with_message(Cow::Borrowed("Username can only contain letters, numbers, _ and -")));
    }
    
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let password = password.trim();
    
    if password.len() < 6 || password.len() > 65 {
        return Err(ValidationError::new("invalid_length")
            .with_message(Cow::Borrowed("Password must be between 6 and 65 characters")));
    }
    
    if !password.is_ascii() {
        return Err(ValidationError::new("non_ascii")
            .with_message(Cow::Borrowed("Only English characters allowed")));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !has_uppercase || !has_lowercase || !has_digit || has_special {
        return Err(ValidationError::new("weak_password")
            .with_message(Cow::Borrowed(
                "Password must contain uppercase, lowercase, digit and no special characters"
            )));
    }
    
    Ok(())
}