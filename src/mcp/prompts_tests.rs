//! Unit tests for src/mcp/prompts.rs

use super::*;

#[test]
fn list_prompts_returns_quick_start() {
    let result = list_prompts();
    let names: Vec<&str> = result.prompts.iter().map(|p| p.name.as_str()).collect();
    assert!(
        names.contains(&"quick_start"),
        "expected quick_start prompt"
    );
}

#[test]
fn get_prompt_quick_start_returns_message() {
    let result = get_prompt(rmcp::model::GetPromptRequestParams::new("quick_start"))
        .expect("quick_start should resolve");
    assert!(
        !result.messages.is_empty(),
        "prompt should have at least one message"
    );
}

#[test]
fn get_prompt_unknown_returns_err() {
    let result = get_prompt(rmcp::model::GetPromptRequestParams::new("nonexistent"));
    assert!(result.is_err(), "unknown prompt should return Err");
}

/// SEP-2549 requires `ttlMs`/`cacheScope` on `prompts/list` at protocol version
/// `2026-07-28`. `..Default::default()` leaves both `None`, which serializes as
/// absent and makes a spec-strict client reject the whole result.
#[test]
fn list_prompts_carries_sep_2549_cache_hints() {
    let result = list_prompts();
    assert_eq!(
        result.ttl_ms,
        Some(PROMPTS_LIST_TTL_MS),
        "prompts/list must carry ttlMs (SEP-2549)"
    );
    assert_eq!(
        result.cache_scope,
        Some(rmcp::model::CacheScope::Public),
        "prompts/list must carry cacheScope (SEP-2549)"
    );
}
