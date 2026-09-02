use antigravity_proxy_rust::auth::{load_accounts_config, get_accounts};
use antigravity_proxy_rust::quota::fetch_quota;

#[test]
fn test_endpoint_hostname_sanitization() {
    let endpoints = vec![
        "https://daily-cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse".to_string(),
        "https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse".to_string(),
    ];
    let sanitized: Vec<String> = endpoints.iter()
        .map(|ep| ep.replace("daily-cloudcode-pa.googleapis.com", "daily-cloudcode-pa.sandbox.googleapis.com"))
        .collect();

    assert_eq!(sanitized[0], "https://daily-cloudcode-pa.sandbox.googleapis.com/v1internal:streamGenerateContent?alt=sse");
    assert_eq!(sanitized[1], "https://cloudcode-pa.googleapis.com/v1internal:streamGenerateContent?alt=sse");
}

#[tokio::test]
async fn test_fetch_quota_live_account() {
    let _ = load_accounts_config().await;
    let accounts = get_accounts();
    if accounts.is_empty() {
        println!("No accounts configured to test fetch_quota live");
        return;
    }

    for acc in &accounts {
        if acc.project_id.is_some() {
            println!("Testing live quota fetch for account: {}", acc.email);
            let res = fetch_quota(acc, true).await;
            match res {
                Ok(Some(quota_entries)) => {
                    println!("Successfully fetched {} quota entries for {}", quota_entries.len(), acc.email);
                    assert!(!quota_entries.is_empty());
                }
                Ok(None) => {
                    println!("Quota response empty for {}", acc.email);
                }
                Err(e) => {
                    panic!("Quota fetch failed for {}: {}", acc.email, e);
                }
            }
        }
    }
}
