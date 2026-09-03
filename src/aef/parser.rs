mod checksum;
mod method_tokens;
mod parse;

/// Parser for Atipicial AEF containers.
///
/// This type is stateless and can be reused across many parse calls.
#[derive(Debug, Default, Clone, Copy)]
pub struct AefParser;

impl AefParser {
    /// Create a new AEF parser.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}
