use antigravity_proxy_rust::utils::{
    detect_loop, clean_json_schema_for_antigravity, parse_google_error,
    transform_to_google_body, convert_openai_response_to_anthropic,
    transform_google_event_to_openai, StreamState
};
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

#[test]
fn test_parse_google_error_model_turn_not_supported() {
    let error_text = r#"{
        "error": {
            "code": 400,
            "message": "Requests ending with a model turn are not supported.",
            "status": "INVALID_ARGUMENT"
        }
    }"#;

    let parsed = parse_google_error(error_text);
    assert!(!parsed.is_model_unsupported, "Should not flag model as unsupported");
    assert_eq!(parsed.status, 400);
    assert_eq!(parsed.reason, "invalid_argument");
}

#[test]
fn test_transform_to_google_body_ending_with_model_turn() {
    let req = json!({
        "model": "gemini-3.7-flash",
        "messages": [
            { "role": "user", "content": "Hello" },
            { "role": "assistant", "content": "I am thinking about..." }
        ]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    let contents = transformed["request"]["contents"].as_array().unwrap();
    assert!(contents.len() >= 3);
    assert_eq!(contents.first().unwrap()["role"], "user");
    assert_eq!(contents.last().unwrap()["role"], "user");
    assert_eq!(contents.last().unwrap()["parts"][0]["text"], "Continue");
}

#[test]
fn test_transform_to_google_body_only_assistant_turn() {
    let req = json!({
        "model": "gemini-3.7-flash",
        "messages": [
            { "role": "assistant", "content": "Here is what I have so far" }
        ]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    let contents = transformed["request"]["contents"].as_array().unwrap();
    assert_eq!(contents.len(), 3);
    assert_eq!(contents[0]["role"], "user");
    assert_eq!(contents[1]["role"], "model");
    assert_eq!(contents[2]["role"], "user");
    assert_eq!(contents[2]["parts"][0]["text"], "Continue");
}

#[test]
fn test_transform_to_google_body_function_call_has_thought_signature() {
    let req = json!({
        "model": "gemini-3.7-flash",
        "messages": [
            { "role": "user", "content": "Please read the file." },
            {
                "role": "assistant",
                "content": null,
                "tool_calls": [
                    {
                        "id": "call_12345",
                        "type": "function",
                        "function": {
                            "name": "read",
                            "arguments": "{\"path\":\"foo.txt\"}"
                        }
                    }
                ]
            },
            {
                "role": "tool",
                "tool_call_id": "call_12345",
                "name": "read",
                "content": "file contents"
            }
        ]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    let contents = transformed["request"]["contents"].as_array().unwrap();
    
    // Find the model message with the functionCall
    let model_msg = contents.iter().find(|m| m["role"] == "model").expect("Model message should exist");
    let parts = model_msg["parts"].as_array().expect("Parts should be array");
    let func_part = parts.iter().find(|p| p.get("functionCall").is_some()).expect("functionCall part should exist");
    
    assert!(func_part.get("thoughtSignature").is_some(), "thoughtSignature must be present on functionCall part");
    assert!(func_part.get("thought_signature").is_some(), "thought_signature must be present on functionCall part");
    assert_eq!(func_part["thoughtSignature"], "skip_thought_signature_validator");
    assert_eq!(func_part["thought_signature"], "skip_thought_signature_validator");
}

#[test]
fn test_transform_to_google_body_function_call_with_sig_id() {
    let req = json!({
        "model": "gemini-3.7-flash",
        "messages": [
            { "role": "user", "content": "Run tool" },
            {
                "role": "assistant",
                "content": null,
                "tool_calls": [
                    {
                        "id": "sig-my_secret_token_123-call_abc456",
                        "type": "function",
                        "function": {
                            "name": "read",
                            "arguments": "{\"path\":\"foo.txt\"}"
                        }
                    }
                ]
            }
        ]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    let contents = transformed["request"]["contents"].as_array().unwrap();
    
    let model_msg = contents.iter().find(|m| m["role"] == "model").expect("Model message should exist");
    let parts = model_msg["parts"].as_array().expect("Parts should be array");
    let func_part = parts.iter().find(|p| p.get("functionCall").is_some()).expect("functionCall part should exist");
    
    assert_eq!(func_part["thoughtSignature"], "my_secret_token_123");
    assert_eq!(func_part["thought_signature"], "my_secret_token_123");
}

#[test]
fn test_transform_google_event_to_openai_cached_tokens() {
    let mut state = StreamState { images_appended: std::collections::HashSet::new() };
    let google_event = json!({
        "response": {
            "candidates": [{
                "content": {
                    "parts": [{ "text": "Hello world" }]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 1500,
                "candidatesTokenCount": 50,
                "totalTokenCount": 1550,
                "cachedContentTokenCount": 1200
            }
        }
    });

    let chunk = transform_google_event_to_openai(&google_event, "gemini-3.7-flash", "chatcmpl-test", false, &mut state)
        .expect("Chunk should transform successfully");

    let usage = chunk.usage.expect("Usage should be present");
    assert_eq!(usage["prompt_tokens"], 1500);
    assert_eq!(usage["completion_tokens"], 50);
    assert_eq!(usage["total_tokens"], 1550);
    assert_eq!(usage["prompt_tokens_details"]["cached_tokens"], 1200);
}

#[test]
fn test_convert_openai_response_to_anthropic_cached_tokens() {
    let openai_res = json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "content": "Hello world"
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 2000,
            "completion_tokens": 100,
            "total_tokens": 2100,
            "prompt_tokens_details": {
                "cached_tokens": 1500
            }
        }
    });

    let anth_res = convert_openai_response_to_anthropic(openai_res, "claude-3-7-sonnet");
    let usage = anth_res["usage"].clone();
    assert_eq!(usage["input_tokens"], 500); // 2000 - 1500
    assert_eq!(usage["output_tokens"], 100);
    assert_eq!(usage["cache_read_input_tokens"], 1500);
    assert_eq!(usage["cache_creation_input_tokens"], 0);
}

#[test]
fn test_transform_to_google_body_no_antigravity_system_instruction() {
    let req = json!({
        "model": "gemini-3.7-flash",
        "messages": [
            { "role": "system", "content": "Custom system instruction" },
            { "role": "user", "content": "Hello" }
        ]
    });

    let transformed = transform_to_google_body(&req, "test-proj", false, None, false);
    let sys_instr = transformed["request"]["system_instruction"].clone();
    let text = sys_instr["parts"][0]["text"].as_str().unwrap();
    assert_eq!(text, "Custom system instruction");
    assert!(!text.contains("You are Antigravity"));
}

#[test]
fn test_append_dataset_record() {
    use antigravity_proxy_rust::utils::append_dataset_record;
    use std::fs;

    let input_messages = vec![
        json!({ "role": "system", "content": "You are a helpful assistant." }),
        json!({ "role": "user", "content": "Hello!" })
    ];

    append_dataset_record("gemini-3.7-flash", &input_messages, "Hi there! How can I help?", "Thinking step 1");

    assert!(std::path::Path::new("captured_dataset.jsonl").exists());
    if let Ok(content) = fs::read_to_string("captured_dataset.jsonl") {
        let lines: Vec<&str> = content.lines().collect();
        assert!(!lines.is_empty());
        let last_line = lines.last().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(last_line).unwrap();
        assert_eq!(parsed["model"], "gemini-3.7-flash");
        let msgs = parsed["messages"].as_array().unwrap();
        assert_eq!(msgs.len(), 3);
        assert_eq!(msgs[2]["role"], "assistant");
        assert_eq!(msgs[2]["content"], "Hi there! How can I help?");
        assert_eq!(msgs[2]["reasoning_content"], "Thinking step 1");
    }
}


