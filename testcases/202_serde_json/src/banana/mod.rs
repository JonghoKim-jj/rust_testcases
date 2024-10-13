use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn banana_response() {
    let error_info = json!({
        "errcode": "M_LIMIT_EXCEEDED",
        "error": "Too many requests",
        "retry_after_ms": 3000 * 2,
    });

    println!("errcode: {}", error_info["errcode"]);
    println!("error: {}", error_info["error"]);
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    errcode: String,
    error: String,
    retry_after_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::{banana_response, ErrorResponse};

    #[test]
    fn run_test() {
        banana_response()
    }

    #[test]
    fn another_test() {
        let error_response = ErrorResponse {
            errcode: "M_LIMIT_RATED".to_string(),
            error: "Rate limited. Try later again".to_string(),
            retry_after_ms: 2000,
        };

        let json = serde_json::to_string(&error_response);

        assert!(json.is_ok())
    }
}