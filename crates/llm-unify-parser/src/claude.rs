//! Claude export parser (stub implementation)

use llm_unify_core::{Conversation, Provider, ProviderTrait};

pub struct ClaudeParser;

impl ProviderTrait for ClaudeParser {
    fn parse(&self, _data: &[u8]) -> llm_unify_core::Result<Vec<Conversation>> {
        // TODO: Implement Claude export parsing
        Ok(Vec::new())
    }

    fn name(&self) -> &'static str {
        "Claude"
    }

    fn validate(&self, _conversation: &Conversation) -> llm_unify_core::Result<()> {
        Ok(())
    }
}
