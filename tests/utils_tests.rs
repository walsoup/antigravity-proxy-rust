use antigravity_proxy_rust::utils::{detect_loop, clean_json_schema_for_antigravity, parse_google_error, transform_to_google_body, convert_openai_response_to_anthropic};
use serde_json::json;

#[test]
fn test_loop_detector_no_loop() {
    let text = "This is a normal message that does not contain any loops or repeating patterns of text.";
    assert!(!detect_loop(text));
}

#[test]
fn test_loop_detector_with_loop() {
    // abcde repeated 10 times -> pattern_size 5, min_repeats 10
    let text = "abcde".repeat(10);
    assert!(detect_loop(&text));
}

#[test]
fn test_loop_detector_short_loop() {
    let text = "a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a";
    assert!(detect_loop(text));
}

#[test]
fn test_schema_cleaning() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "some description text" // should be removed in aggressive mode
            },
            "age": {
                "type": "integer"
            }
        },
        "required": ["name"]
    });
    
    let cleaned = clean_json_schema_for_antigravity(schema, true);
    
    // Check that "description" key is removed
    assert!(cleaned.get("properties").unwrap().get("name").unwrap().get("description").is_none());
}

#[test]
fn test_parse_google_error_captcha() {
    let error_text = r#"{
        "error": {
            "code": 403,
            "message": "Please validate your identity via CAPTCHA challenge",
            "status": "PERMISSION_DENIED",
            "details": [
                {
                    "reason": "VALIDATION_REQUIRED",
                    "validation_url": "https://google.com/challenge"
                }
            ]
        }
    }"#;
    
    let parsed = parse_google_error(error_text);
    assert!(parsed.is_challenge_required);
    assert_eq!(parsed.validation_url, Some("https://google.com/challenge".to_string()));
}

#[test]
fn test_gemini_3_6_model_transformation() {
    let req = json!({
        "model": "gemini-3.6-flash-high",
        "messages": [{"role": "user", "content": "hello"}]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    assert_eq!(transformed.get("model").unwrap().as_str().unwrap(), "gemini-3.6-flash-high");

    let req_medium = json!({
        "model": "gemini-3.6-flash",
        "reasoning_effort": "medium",
        "messages": [{"role": "user", "content": "hello"}]
    });

    let transformed_med = transform_to_google_body(&req_medium, "test-proj", false, None, false);
    assert_eq!(transformed_med.get("model").unwrap().as_str().unwrap(), "gemini-3.6-flash-medium");
}

#[test]
fn test_convert_openai_response_to_anthropic_empty_content() {
    let empty_res = json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "content": ""
            },
            "finish_reason": "stop"
        }]
    });

    let anth_res = convert_openai_response_to_anthropic(empty_res, "claude-3-5-sonnet");
    let content = anth_res.get("content").unwrap().as_array().unwrap();
    assert!(!content.is_empty(), "Anthropic content array must not be empty");
    assert_eq!(content[0].get("type").unwrap().as_str().unwrap(), "text");
}
