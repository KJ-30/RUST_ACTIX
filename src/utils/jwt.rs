use super::constants;
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::future;

/// Claims 结构体用于JWT中，以携带用户信息
#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub exp: usize,    // 过期时间
    pub iat: usize,    // 签发时间
    pub email: String, // 用户邮箱
    pub id: i32,
}

/// 实现 FromRequest 以便在请求中提取 Claims 信息
impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;
    /// 从请求中提取 Claims 信息
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Self, Self::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claims) => future::ready(Ok(claims.clone())),
            None => future::ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}

/// 生成 JWT Token
pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        exp: (now + Duration::days(1)).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };
    let jwt_secret = (*constants::JWT_SECRET).clone();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

/// 验证并解析 JWT Token
pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = (*constants::JWT_SECRET).clone();
    let claim_data = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );
    claim_data
}