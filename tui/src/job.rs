use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::compositor::Compositor;
use futures_util::future::{BoxFuture, Future, FutureExt};
use futures_util::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::OnceCell;
use log::log;
use crate::status::StatusMessage;

static JOB_QUEUE:OnceCell<Sender<Callback>> = OnceCell::new();
pub async fn dispatch_callback(job: Callback) {
    let _ = JOB_QUEUE.wait().send(job).await;
}
pub type EditorCompositorCallback = Box<dyn FnOnce(&mut Compositor) + Send>;

/// Compositor 的回调 以及 Editor本身的回调
pub enum Callback {
    EditorCompositor(EditorCompositorCallback),
}

pub type JobFuture = BoxFuture<'static, anyhow::Result<Option<Callback>>>;

pub struct Job {
    pub future: BoxFuture<'static, anyhow::Result<Option<Callback>>>,
    pub wait_before_exit: bool,
}
pub struct Jobs {
    /// jobs that need to complete before we exit.
    pub wait_futures: FuturesUnordered<JobFuture>,
    pub callbacks: Receiver<Callback>,
}

impl Job {
    pub fn new<F: Future<Output = anyhow::Result<()>> + Send + 'static>(f: F) -> Self {
        Self {
            future: f.map(|r| r.map(|()| None)).boxed(),
            wait_before_exit: false,
        }
    }

    pub fn with_callback<F: Future<Output = anyhow::Result<Callback>> + Send + 'static>(
        f: F,
    ) -> Self {
        Self {
            future: f.map(|r| r.map(Some)).boxed(),
            wait_before_exit: false,
        }
    }

    pub fn wait_before_exiting(mut self) -> Self {
        self.wait_before_exit = false;
        self
    }
}
impl Jobs {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = channel(1024);
        let _ = JOB_QUEUE.set(tx);
        Self {
            wait_futures: FuturesUnordered::new(),
            callbacks: rx,
        }
    }

    pub fn spawn<F: Future<Output = anyhow::Result<()>> + Send + 'static>(&mut self, f: F) {
        self.add(Job::new(f));
    }

    pub fn callback<F: Future<Output = anyhow::Result<Callback>> + Send + 'static>(
        &mut self,
        f: F,
    ) {
        self.add(Job::with_callback(f));
    }

    pub fn handle_callback(
        &self,
        compositor: &mut Compositor,
        call: anyhow::Result<Option<Callback>>,
    ) {
        match call {
            Ok(None) => {}
            Ok(Some(call)) => match call {
                Callback::EditorCompositor(call) => call( compositor),
            },
            Err(e) => {

            }
        }
    }

    pub fn add(&self, j: Job) {
        if j.wait_before_exit {
            self.wait_futures.push(j.future);
        } else {
            tokio::spawn(async move {
                match j.future.await {
                    Ok(Some(cb)) => dispatch_callback(cb).await,
                    Ok(None) => (),
                    Err(err) => (),
                }
            });
        }
    }

    /// Blocks until all the jobs that need to be waited on are done.
    pub async fn finish(
        &mut self,
        mut compositor: Option<&mut Compositor>,
    ) -> anyhow::Result<()> {
        log::debug!("waiting on jobs...");
        let mut wait_futures = std::mem::take(&mut self.wait_futures);

        while let (Some(job), tail) = wait_futures.into_future().await {
            match job {
                Ok(callback) => {
                    wait_futures = tail;

                    if let Some(callback) = callback {
                        // clippy doesn't realize this is an error without the derefs
                        #[allow(clippy::needless_option_as_deref)]
                        match callback {
                            Callback::EditorCompositor(call) if compositor.is_some() => {
                                call(compositor.as_deref_mut().unwrap())
                            }
                            // skip callbacks for which we don't have the necessary references
                            _ => (),
                        }
                    }
                }
                Err(e) => {
                    self.wait_futures = tail;
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
