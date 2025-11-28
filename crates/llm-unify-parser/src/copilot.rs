//! GitHub Copilot export parser (stub implementation)

use llm_unify_core::{Conversation, Provider, ProviderTrait};

pub struct CopilotParser;

impl ProviderTrait for CopilotParser {
    fn parse(&self, _data: &[u8]) -> llm_unify_core::Result<Vec<Conversation>> {
        // TODO: Implement Copilot export parsing
        Ok(Vec::new())
    }

    fn name(&self) -> &'static str {
        "Copilot"
    }

    fn validate(&self, _conversation: &Conversation) -> llm_unify_core::Result<()> {
        Ok(())
    }
}
