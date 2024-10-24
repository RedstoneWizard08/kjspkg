use std::{
    process::exit,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use tokio::task::AbortHandle;

lazy_static! {
    pub static ref ABORT_HANDLES: Arc<Mutex<Vec<AbortHandle>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn register_exit_handler() -> Result<()> {
    ctrlc::set_handler(move || {
        let handles = ABORT_HANDLES.lock().unwrap();
        let handles = handles.iter();

        for handle in handles {
            handle.abort();
        }

        exit(130);
    })
    .map_err(|v| v.into())
}
