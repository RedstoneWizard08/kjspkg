mod err;
mod util;

pub use err::*;
pub use util::*;

pub type Result<T, E = AppError> = core::result::Result<T, E>;
