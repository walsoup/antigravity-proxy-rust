use antigravity_proxy_rust::utils::transform_to_google_body;
use serde_json::json;

#[test]
fn test_gemini_3_8_flash_model_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-high");
}

#[test]
fn test_gemini_3_8_flash_medium_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash-medium",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-medium");
}

#[test]
fn test_gemini_3_8_flash_low_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash-low",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-low");
}

#[test]
fn test_gemini_3_7_flash_model_resolution() {
    let openai_body = json!({
        "model": "gemini-3.7-flash",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.7-flash-tiered");
}

#[test]
fn test_gemini_3_7_flash_high_resolution() {
    let openai_body = json!({
        "model": "gemini-3.7-flash-high",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.7-flash-tiered");
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"], "high");
}

#[test]
fn test_gemini_3_7_flash_tiered_resolution() {
    let openai_body = json!({
        "model": "gemini-3.7-flash-tiered",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.7-flash-tiered");
}

#[test]
fn test_gemini_3_6_flash_model_resolution() {
    let openai_body = json!({
        "model": "gemini-3.6-flash",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.6-flash-high");
}

#[test]
fn test_gemini_3_6_flash_low_resolution() {
    let openai_body = json!({
        "model": "gemini-3.6-flash-low",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.6-flash-low");
}

#[test]
fn test_gpt_oss_model_resolution() {
    let openai_body = json!({
        "model": "gpt-oss-120b-medium",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gpt-oss-120b-medium");
}

#[test]
fn test_proactive_observer_resolution() {
    let openai_body = json!({
        "model": "proactive-observer",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "models/proactive-observer");
}

#[test]
fn test_m50_resolution() {
    let openai_body = json!({
        "model": "m50",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.1-flash-lite");
}
