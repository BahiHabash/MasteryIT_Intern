use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(pub anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("Handler error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
