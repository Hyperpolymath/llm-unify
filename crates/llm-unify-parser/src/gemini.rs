//! Gemini export parser (stub implementation)

use llm_unify_core::{Conversation, Provider, ProviderTrait};

pub struct GeminiParser;

impl ProviderTrait for GeminiParser {
    fn parse(&self, _data: &[u8]) -> llm_unify_core::Result<Vec<Conversation>> {
        // TODO: Implement Gemini export parsing
        Ok(Vec::new())
    }

    fn name(&self) -> &'static str {
        "Gemini"
    }

    fn validate(&self, _conversation: &Conversation) -> llm_unify_core::Result<()> {
        Ok(())
    }
}
