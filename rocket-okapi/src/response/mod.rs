mod responder_impls;

use super::gen::OpenApiGenerator;
use super::Result;
use okapi::openapi3::Responses;

/// No missing documentation here officer, no siree.
pub trait OpenApiResponder {
    /// Create the responses type, which is a list of responses that can be
    /// rendered in `openapi.json` format.
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses>;
}
