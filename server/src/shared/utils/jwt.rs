use actix_web::http::{StatusCode, header::HeaderMap};
use chrono::{Duration, Utc, DateTime};
use jsonwebtoken::{Header, Validation, TokenData, encode, decode, EncodingKey, DecodingKey};
use sea_orm::{EntityTrait, DatabaseConnection};
use serde_json::to_value;
use uuid::Uuid;

use shared_types::auth_dtos::{RefreshTokenDTO, Claims};

use crate::config::Config;
use crate::models::user_model::{Entity as UserEntity, Model as UserModel};
use crate::models::refresh_token_model::Model as RefreshTokenModel;
use crate::shared::utils::errors::{ServerError, HttpError};

pub async fn validate_jwt(headers: &HeaderMap, secret_key: &str, validation: &Validation, db: &DatabaseConnection) -> Result<(), ServerError> {
    if let Some(token_header) = headers.get("Authorization") {
        let token_str = token_header.to_str().unwrap_or("");

        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

        let token_data : TokenData<Claims> = decode(token_str, &decoding_key, validation).map_err(ServerError::from)?;

        let uid = token_data.claims.sub.parse::<Uuid>()
            .map_err(ServerError::UuidError)?;

        let found_user = UserEntity::find_by_id(uid)
            .one(db).await
            .map_err(|_| ServerError::WebError(HttpError {
                status: StatusCode::NOT_FOUND,
                message: "Database error".to_string(),
            }))?;

        if let Some(_found_user) = found_user {
        // TODO:: Authentication success
            Ok(())
        } else {
            Err(ServerError::UserNotFound)
        }   

    } else {
        Err(ServerError::WebError(HttpError {
            status: StatusCode::UNAUTHORIZED,
            message: "No Authorization header".to_string(),
        }))
    }
}

pub fn create_access_token(user: UserModel, configs: &Config) -> Result<String, ServerError> {
    let secret_key = &configs.secret_key;
    let jwt_iss = configs.jwt_issuer.clone();
    let jwt_aud = configs.jwt_audience.clone();

    let user_id = user.id.to_string();
    let user_data = match to_value(&user) {
        Ok(json) => Some(json),
        Err(err) => return Err(ServerError::JsonError(err))
    };

    let now: DateTime<Utc> = Utc::now();
    let expiry: DateTime<Utc> = Utc::now() + Duration::hours(1);

    let claims = Claims {
        sub:user_id,
        iat:now,
        exp:expiry,
        iss:jwt_iss,
        aud:jwt_aud,
        data:user_data,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()));
    match token {
        Ok(token) => Ok(token),
        Err(err) => Err(ServerError::from(err)),
    }
}

pub fn create_refresh_token(user_id: String, configs: &Config) -> Result<RefreshTokenDTO, ServerError> {
    let jwt_iss = configs.jwt_issuer.clone();
    let jwt_aud = configs.jwt_audience.clone();
    let secret_key = &configs.secret_key;
    let now: DateTime<Utc> = Utc::now();
    let expiry: DateTime<Utc> = now + Duration::hours(12);

    let claims = Claims {
        sub: user_id,
        iat: now,
        exp: expiry,
        iss: jwt_iss.clone(),
        aud: jwt_aud.clone(),
        data: None,
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))?;
    
    let refresh_token_dto = RefreshTokenDTO {
        refresh_token: token,
        issued_at: now,
        expires_at: expiry,
        jwt_iss,
        jwt_aud,
        revoked: false
    };

    Ok(refresh_token_dto)
}


pub async fn refresh_access_token_util(refresh_token: RefreshTokenModel, db: &DatabaseConnection, configs: &Config) -> Result<String, ServerError> {
    let secret_key = configs.secret_key.as_bytes();
  
    let decoded_token = decode::<Claims>(
        &refresh_token.token,&DecodingKey::from_secret(secret_key),&Validation::default(),)
        .map_err(ServerError::from)?;

    let user_id = decoded_token.claims.sub;
    
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(err) => return Err(ServerError::UuidError(err)),
    };
    
    let users = UserEntity::find_by_id(uuid).all(db).await?;
    
    let user = match users.first() {
        Some(user) => user.clone(),
        None => return Err(ServerError::UserNotFound),
    };

    let access_token = create_access_token(user, configs)?;

    Ok(access_token)
}
