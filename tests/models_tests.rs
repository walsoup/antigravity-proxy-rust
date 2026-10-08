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

    // ...while reasoning-off must omit the key: live-verified 2026-09-07
    // that omit returns 200 with ZERO thought traces on lite (bare 0 400s,
    // and 0-with-level is ignored by the server, reasoning fully).
    let body = json!({
        "model": "gemini-3.5-flash-lite",
        "reasoning_effort": "none",
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.5-flash-lite");
    assert!(
        google_req["request"]["generationConfig"].get("thinkingConfig").is_none(),
        "lite reasoning-off must omit thinkingConfig (live: 200 with zero traces)"
    );

    // Same via -none suffix and via explicit thinking_budget 0: the key must
    // be omitted (never emit bare 0 — the server 400s it).
    for body in [
        json!({
            "model": "gemini-3.5-flash-lite-none",
            "messages": [{ "role": "user", "content": "Hello" }]
        }),
        json!({
            "model": "gemini-3.5-flash-lite",
            "thinking_budget": 0,
            "messages": [{ "role": "user", "content": "Hello" }]
        }),
    ] {
        let google_req = transform_to_google_body(&body, "test-project", false, None, false);
        assert_eq!(google_req["model"], "gemini-3.5-flash-lite");
        assert!(
            google_req["request"]["generationConfig"].get("thinkingConfig").is_none(),
            "lite reasoning-off must omit thinkingConfig (bare 0 is rejected upstream)"
        );
    }
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
    // xhigh (and max) map above high (32768) while staying below the
    // live-verified server ceiling of 65535, and select the high-tier model.
    for effort in ["xhigh", "max"] {
        let openai_body = json!({
            "model": "gemini-3.8-flash",
            "reasoning_effort": effort,
            "messages": [{ "role": "user", "content": "Hello" }]
        });

        let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
        assert_eq!(google_req["model"], "gemini-3.8-flash-high");
        assert_eq!(
            google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"],
            49152,
            "effort {effort} must map above high but below the server ceiling"
        );
        assert_eq!(
            google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"],
            "high"
        );
        // xhigh/max carry the compact extended-reasoning directive (the only
        // lever that induces MORE thinking: budget is a cap, level is maxed).
        let sys_text = google_req["request"]["systemInstruction"]["parts"][0]["text"].as_str().unwrap();
        assert!(sys_text.contains("[REASONING DEPTH: MAXIMUM]"), "effort {effort} must carry the extended-reasoning directive");
    }
}

#[test]
fn test_gemini_3_8_flash_max_suffix_resolution() {
    let openai_body = json!({
        "model": "gemini-3.8-flash-max",
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-3.8-flash-high");
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], 49152);
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"], "high");
    let sys_text = google_req["request"]["systemInstruction"]["parts"][0]["text"].as_str().unwrap();
    assert!(sys_text.contains("[REASONING DEPTH: MAXIMUM]"), "max suffix must carry the extended-reasoning directive");
}

#[test]
fn test_gemini_3_5_flash_high_vs_xhigh_reasoning_ladder() {
    // xhigh extends native high via larger budget PLUS the compact
    // extended-reasoning directive (same model, same native level "high").
    // high itself stays fully native.
    for (high_model, xhigh_model) in [
        ("gemini-3.5-flash-high", "gemini-3.5-flash-xhigh"),
        ("gemini-3.5-flash-lite-high", "gemini-3.5-flash-lite-xhigh"),
    ] {
        let mk = |model: &str| {
            transform_to_google_body(
                &json!({
                    "model": model,
                    "messages": [{ "role": "user", "content": "Hello" }]
                }),
                "test-project",
                false,
                None,
                false,
            )
        };
        let high = mk(high_model);
        let xhigh = mk(xhigh_model);
        assert_eq!(high["model"], xhigh["model"], "{high_model} and {xhigh_model} must route identically");
        let high_budget = high["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"]
            .as_i64()
            .expect("high must carry thinkingBudget");
        let xhigh_budget = xhigh["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"]
            .as_i64()
            .expect("xhigh must carry thinkingBudget");
        assert!(
            xhigh_budget > high_budget,
            "xhigh budget ({xhigh_budget}) must exceed high budget ({high_budget})"
        );
        let high_sys = high["request"]["systemInstruction"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("");
        let xhigh_sys = xhigh["request"]["systemInstruction"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("");
        assert!(!high_sys.contains("REASONING DIRECTIVE") && !high_sys.contains("REASONING DEPTH"), "{high_model} must stay native (no prompt directive)");
        assert!(xhigh_sys.contains("[REASONING DEPTH: MAXIMUM]"), "{xhigh_model} must carry the extended-reasoning directive");
    }

    // Same ladder via reasoning_effort on the bare model id.
    let mk_effort = |effort: &str| {
        transform_to_google_body(
            &json!({
                "model": "gemini-3.5-flash",
                "reasoning_effort": effort,
                "messages": [{ "role": "user", "content": "Hello" }]
            }),
            "test-project",
            false,
            None,
            false,
        )
    };
    let high = mk_effort("high");
    let xhigh = mk_effort("xhigh");
    assert_eq!(high["model"], "gemini-3-flash-agent");
    assert_eq!(xhigh["model"], "gemini-3-flash-agent");
    assert!(
        xhigh["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"]
            .as_i64()
            .unwrap()
            > high["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"]
                .as_i64()
                .unwrap()
    );
    assert!(!high["request"]["systemInstruction"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .contains("REASONING DIRECTIVE") && !high["request"]["systemInstruction"]["parts"][0]["text"].as_str().unwrap_or("").contains("REASONING DEPTH"), "reasoning_effort high must stay native");
    assert!(xhigh["request"]["systemInstruction"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .contains("[REASONING DEPTH: MAXIMUM]"), "reasoning_effort xhigh must carry the extended-reasoning directive");
}

#[test]
fn test_negative_one_thinking_budget() {
    let openai_body = json!({
        "model": "gemini-3.8-flash",
        "thinking_budget": -1,
        "messages": [{ "role": "user", "content": "Hello" }]
    });

    let google_req = transform_to_google_body(&openai_body, "test-project", false, None, false);
    assert_eq!(google_req["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], -1);
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

#[test]
fn test_max_output_tokens_ceiling_clamping() {
    // gemini-3.5-flash-lite must clamp 65536 down to 65535
    let lite_body = json!({
        "model": "gemini-3.5-flash-lite",
        "max_tokens": 65536,
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&lite_body, "test-project", false, None, false);
    assert_eq!(google_req["request"]["generationConfig"]["maxOutputTokens"], 65535);

    // claude must clamp down to 64000
    let claude_body = json!({
        "model": "claude-sonnet-4-6",
        "max_tokens": 65536,
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&claude_body, "test-project", false, None, false);
    assert_eq!(google_req["request"]["generationConfig"]["maxOutputTokens"], 64000);

    // max_completion_tokens is also recognized and clamped
    let comp_body = json!({
        "model": "gemini-3.5-flash-lite",
        "max_completion_tokens": 70000,
        "messages": [{ "role": "user", "content": "Hello" }]
    });
    let google_req = transform_to_google_body(&comp_body, "test-project", false, None, false);
    assert_eq!(google_req["request"]["generationConfig"]["maxOutputTokens"], 65535);
}

#[test]
fn test_future_gemini_4_model_resolution_and_thinking() {
    let body = json!({
        "model": "gemini-4-flash",
        "messages": [{ "role": "user", "content": "Hello future" }]
    });
    let google_req = transform_to_google_body(&body, "test-project", false, None, false);
    assert_eq!(google_req["model"], "gemini-4-flash");
    
    // Future gemini models must automatically receive thinkingConfig
    let thinking_config = &google_req["request"]["generationConfig"]["thinkingConfig"];
    assert!(thinking_config.get("thinkingBudget").is_some());
    assert_eq!(thinking_config["thinkingLevel"], "low");

    // Explicit reasoning effort on future models
    let high_body = json!({
        "model": "gemini-4-pro",
        "reasoning_effort": "high",
        "messages": [{ "role": "user", "content": "Think deep" }]
    });
    let google_req_high = transform_to_google_body(&high_body, "test-project", false, None, false);
    assert_eq!(google_req_high["model"], "gemini-4-pro");
    assert_eq!(google_req_high["request"]["generationConfig"]["thinkingConfig"]["thinkingLevel"], "high");
    assert_eq!(google_req_high["request"]["generationConfig"]["thinkingConfig"]["thinkingBudget"], 32768);
}

#[test]
fn test_claude_55_variant_suffix_preserved() {
    // Live-verified 2026-10-04 (sandbox): 5-5 exists ONLY with -low/-medium/-high;
    // bare id 404s, so the transform must not strip the suffix.
    for model in [
        "claude-sonnet-5-5-low",
        "claude-sonnet-5-5-medium",
        "claude-sonnet-5-5-high",
        "claude-opus-5-5-low",
        "claude-opus-5-5-medium",
        "claude-opus-5-5-high",
    ] {
        let body = json!({
            "model": model,
            "messages": [{ "role": "user", "content": "hi" }]
        });
        let google_req = transform_to_google_body(&body, "test-project", false, None, false);
        assert_eq!(google_req["model"], model, "suffix stripped for {}", model);
    }
}
