use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::utils::{
    api_response::{self, ApiResponse},
    jwt::decode_jwt,
};

/// 检查认证的中间件函数
///
/// 该函数拦截服务请求，检查客户端是否提供了有效的JWT令牌，并将解码后的令牌声明添加到请求扩展中
/// 如果未提供认证信息或认证失败，将返回相应的错误响应
///
/// # Parameters
/// - `req`: 服务请求对象，包含客户端的请求信息
/// - `next`: 一个继续处理当前请求的函数，用于执行请求链中的下一个中间件或最终处理函数
///
/// # Returns
/// - `Ok(ServiceResponse<impl MessageBody>)`: 如果认证成功，继续执行请求链并返回响应
/// - `Err(Error)`: 如果认证失败或出现错误，返回相应的错误响应
pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // 从请求头中获取认证信息
    let auth = req.headers().get(AUTHORIZATION);

    // 如果没有提供认证信息，返回401未授权错误
    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorised".to_string(),
        )));
    }

    // 解析并验证JWT令牌
    let token = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();
    let claim = decode_jwt(token).unwrap();
    req.extensions_mut().insert(claim.claims);

    // 继续执行请求链，如果执行过程中出现错误，返回500服务器内部错误
    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::new(500, err.to_string())))
}
