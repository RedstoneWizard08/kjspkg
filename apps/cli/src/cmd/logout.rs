use crate::ctx::{CliContext, RawCliContext};
use eyre::Result;

pub async fn cmd_logout(_cx: &CliContext) -> Result<()> {
    CliContext::of(RawCliContext {
        api_base: None,
        token: None,
    })?
    .save()?;

    exit(0); // We have to exit or it'll get overwritten.
}
