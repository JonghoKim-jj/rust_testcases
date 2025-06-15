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
    use serde_json::Value;

    #[test]
    fn run_test() {
        banana_response()
    }

    #[test]
    fn struct_to_json_string() {
        let error_response = ErrorResponse {
            errcode: "M_LIMIT_RATED".to_string(),
            error: "Rate limited. Try again later".to_string(),
            retry_after_ms: 2000,
        };

        let json = serde_json::to_string(&error_response);
        let compact_json = r#"{"errcode":"M_LIMIT_RATED","error":"Rate limited. Try again later","retry_after_ms":2000}"#;

        assert!(json.is_ok());
        assert_eq!(json.unwrap(), compact_json);
    }

    #[test]
    fn parse_json_string() {
        let json = r#"{"errcode":"M_LIMIT_RATED","error":"Rate limited. Try again later","retry_after_ms":2000}"#;
        let v: Value = serde_json::from_str(&json).unwrap();

        assert_eq!(v["errcode"], "M_LIMIT_RATED");
        assert_eq!(v["error"], "Rate limited. Try again later");
        assert_eq!(v["retry_after_ms"], 2000);

        // Two ways of decode using from_value()
        let error_response1: ErrorResponse = serde_json::from_value(v.clone()).unwrap();
        let error_response2 = serde_json::from_value::<ErrorResponse>(v.clone()).unwrap();

        assert_eq!(error_response1.errcode, error_response2.errcode);
        assert_eq!(error_response1.error, error_response2.error);
        assert_eq!(
            error_response1.retry_after_ms,
            error_response2.retry_after_ms
        );
    }
}
