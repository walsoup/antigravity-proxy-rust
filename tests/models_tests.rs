use antigravity_proxy_rust::utils::transform_to_google_body;
use serde_json::json;

#[test]
fn test_gemini_3_8_flash_model_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-tiered");
}

#[test]
fn test_gemini_3_8_flash_tiered_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash-tiered",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-tiered");
}

#[test]
fn test_gemini_3_5_flash_lite_resolution() {
    let openai_body = json!({
        "model": "gemini-3.5-flash-lite",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.5-flash-lite");
}

#[test]
fn test_gemini_3_5_flash_lite_thinking_rules() {
    // M277 400s ONLY on thinkingBudget 0; every other shape passes (live-verified).
    // Default path must carry a nonzero budget...
    let body = json!({
        "model": "gemini-3.5-flash-lite",
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.5-flash-lite");
    let budget = google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"]
        .as_u64()
        .expect("lite default must carry thinkingBudget");
    assert!(budget > 0, "lite thinkingBudget must never be 0");

    // ...while reasoning-off must omit the key instead of forcing 0.
    let body = json!({
        "model": "gemini-3.5-flash-lite",
        "reasoning_effort": "none",
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.5-flash-lite");
    assert!(
        google_req["request"]["generationConfig"].get("thinkingConfig").is_none(),
        "lite reasoning-off must omit thinkingConfig (0 is rejected upstream)"
    );
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
    // -low suffix selects the variant; with no reasoning_effort the default budget applies.
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], 16000);
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"], "low");
}

#[test]
fn test_xhigh_effort_max_budget() {
    // xhigh (and max) map to the live-verified server ceiling of 65535.
    for effort in ["xhigh", "max"] {
        let openai_body = json!({
            "model": "gemini-3.8-flash",
            "reasoning_effort": effort,
            "messages": [{ "role": "user", "content": "Hello" }]
        });

        let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
        assert_eq!(
            google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"],
            65535,
            "effort {effort} must map to max budget"
        );
        assert_eq!(
            google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"],
            "high"
        );
    }
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

#[test]
fn test_gemini_3_8_flash_no_reasoning_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash",
        "reasoning_effort": "none",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-low");
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], 0);
}

#[test]
fn test_gemini_3_8_flash_none_suffix_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash-none",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-low");
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], 0);
}
