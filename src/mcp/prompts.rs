//! MCP prompts for the yarr server.
//!
//! Prompts are pre-canned message templates that MCP clients can invoke.
//! They appear in the "Prompts" section of compatible MCP UIs.
//!
//! Add Yarr-specific prompts here when recurring operator workflows become
//! worth exposing directly to MCP clients.

use rmcp::model::{
    CacheScope, GetPromptRequestParams, GetPromptResult, ListPromptsResult, Prompt, PromptMessage,
    Role,
};

/// SEP-2549 (`ttlMs`/`cacheScope`) is required on `prompts/list` for clients that
/// negotiate protocol version `2026-07-28` (see `rmcp_server.rs`'s `list_tools`
/// for the sibling fix). The prompt list is static per binary, so a long TTL is
/// safe.
const PROMPTS_LIST_TTL_MS: u64 = 600_000;

pub(super) fn list_prompts() -> ListPromptsResult {
    ListPromptsResult {
        prompts: vec![Prompt::new(
            "quick_start",
            Some(
                "Write a short Code Mode script that discovers a configured service \
                 and fetches its status to verify the MCP connection end-to-end.",
            ),
            None,
        )],
        ..Default::default()
    }
    .with_ttl_ms(PROMPTS_LIST_TTL_MS)
    .with_cache_scope(CacheScope::Public)
}

pub(super) fn get_prompt(request: GetPromptRequestParams) -> anyhow::Result<GetPromptResult> {
    match request.name.as_str() {
        "quick_start" => Ok(GetPromptResult::new(vec![PromptMessage::new_text(
            Role::User,
            "Call the `yarr` tool with a Code Mode script. Inside it, use \
             codemode.search('status') to find a service's status callable, then invoke it \
             (e.g. `await sonarr.service_status()`) and return the result. The service is baked \
             into each callable, so you never pass a `service` argument. Report back what you found.",
        )])
        .with_description("Verify Yarr MCP connectivity with a Code Mode discovery + status call")),
        other => Err(anyhow::anyhow!("unknown prompt: {other}")),
    }
}

#[cfg(test)]
#[path = "prompts_tests.rs"]
mod tests;
