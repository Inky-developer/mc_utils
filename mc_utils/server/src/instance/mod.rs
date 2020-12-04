mod error;
pub use error::{InstanceError, Result};

mod implementation;
pub use implementation::{ServerBuilder, ServerInstance};

mod handle;
pub use handle::run_server;
