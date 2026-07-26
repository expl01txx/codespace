use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::rngs::OsRng;  // Вот правильный импорт

pub struct HashUtil {
    argon2: Argon2<'static>,
}

impl HashUtil {
    pub fn new() -> Self {
        HashUtil {
            argon2: Argon2::default(),
        }
    }
    
    pub fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?;
        
        Ok(hash.to_string())
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| e.to_string())?;
        
        Ok(self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}