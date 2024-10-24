use std::process::ExitStatus;

use axum::Router;
use tokio::{spawn, task::JoinHandle};

use crate::{
    abort::ABORT_HANDLES,
    config::GlueConfig,
    framework::Framework,
    router::{register_embedded, register_proxy},
    runner::start_client,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Glue {
    opts: GlueConfig,
}

impl Glue {
    pub fn new(opts: GlueConfig) -> Self {
        Self { opts }
    }

    pub fn register<T>(&self, router: Router<T>, use_proxy: bool) -> Router<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        if use_proxy {
            register_proxy(
                self.opts.clone().base.unwrap(),
                router,
                self.opts.clone().framework,
            )
        } else {
            register_embedded(self.opts.clone().dir.unwrap(), router)
        }
    }

    pub async fn start(&self) -> ExitStatus {
        start_client(
            self.opts.project.clone().unwrap(),
            self.opts.cmd.clone(),
            self.opts.framework.unwrap_or(Framework::None),
        )
        .await
    }

    pub async fn spawn(&self) -> JoinHandle<ExitStatus> {
        let this = self.clone();
        let thread = spawn(async move { this.start().await });
        let handle = thread.abort_handle();

        ABORT_HANDLES.lock().unwrap().push(handle);

        thread
    }
}
