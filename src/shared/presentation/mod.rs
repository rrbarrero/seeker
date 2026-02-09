pub mod openapi;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_response_serialization() {
        let error = ApiErrorResponse {
            message: "Something went wrong".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert_eq!(json, r#"{"message":"Something went wrong"}"#);
    }
}
