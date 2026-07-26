use sea_orm::*;
use serde::Deserialize;
use validator::Validate;

use crate::models::users;
use crate::errors::app_error::AppError;
use crate::utils::hash::HashUtil;

pub struct AuthService {
    db: DatabaseConnection,
    hash_util: HashUtil,
}

#[derive(Debug, Validate, Deserialize)]
pub struct AuthInput {
    #[validate(length(min = 4, max = 30, message = "Username must be 4-30 characters"))]
    pub username: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

impl AuthService {
    pub fn new(db: DatabaseConnection) -> Self {
        AuthService {
            db,
            hash_util: HashUtil::new(),
        }
    }
    
    pub async fn register(&self, input: AuthInput) -> Result<users::Model, AppError> {
        input.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        let username = input.username.trim().to_string();
        
        let existing = users::Entity::find()
            .filter(users::Column::Username.eq(&username))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;
        
        if existing.is_some() {
            return Err(AppError::BadRequest("Username already taken".to_string()));
        }
        
        let password_hash = self.hash_util
            .hash_password(&input.password)
            .map_err(|e| AppError::InternalError(e))?;
        
        let new_user = users::ActiveModel {
            username: Set(username),
            password: Set(password_hash),
            coins: Set(0),
            is_admin: Set(false),
            ..Default::default()
        };
        
        let user = new_user.insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(user)
    }
    
    pub async fn login(&self, input: AuthInput) -> Result<users::Model, AppError> {
        input.validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        let username = input.username.trim();
        
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or(AppError::Unauthorized("Invalid username or password".to_string()))?;
        
        let is_valid = self.hash_util
            .verify_password(&input.password, &user.password)
            .map_err(|e| AppError::InternalError(e))?;
        
        if !is_valid {
            return Err(AppError::Unauthorized("Invalid username or password".to_string()));
        }
        
        Ok(user)
    }

}