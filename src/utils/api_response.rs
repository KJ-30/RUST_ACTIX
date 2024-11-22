use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};
use std::fmt::Display;

// ApiResponse 结构体用于封装 API 响应信息
#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

// 实现 ApiResponse 构造函数
impl ApiResponse {
    // 创建一个新的 ApiResponse 实例
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            // 根据传入的状态码创建 StatusCode 实例
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

// 实现 Responder trait 以支持 ApiResponse 作为响应返回
impl Responder for ApiResponse {
    type Body = BoxBody;
    // 构建 HttpResponse
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let _ = req;
        // 将 body 转换为 BoxBody 类型
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

// 实现 Display trait 以支持字符串表示错误信息
impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {} \n Status Code: {}",
            self.body, self.status_code
        )
    }
}

// 实现 ResponseError trait 以支持作为错误响应返回
impl ResponseError for ApiResponse {
    // 返回错误响应的状态码
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    // 构建错误响应
    fn error_response(&self) -> HttpResponse<BoxBody> {
        // 将 body 转换为 BoxBody 类型
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}
