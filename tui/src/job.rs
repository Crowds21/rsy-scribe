use crate::compositor::Compositor;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::OnceLock;
use tokio::sync::mpsc::{self, Receiver, Sender};
use view::editor::EditorModel;

type Callback = Box<dyn FnOnce(&mut EditorModel, &mut Compositor) + Send + 'static>;

/// 全局任务队列
pub struct JobQueue {
    pub callbacks: Receiver<Callback>,
}

impl JobQueue {
    pub fn handle_callback(
        &self,
        editor_model: &mut EditorModel,
        compositor: &mut Compositor,
        call: anyhow::Result<Option<Callback>>,
    ) {
        match call {
            Ok(None) => {}
            Ok(Some(call)) => call(editor_model, compositor),
            Err(_) => {}
        }
    }

    pub fn new() -> Self {
        static INSTANCE: OnceLock<JobQueue> = OnceLock::new();
        let (tx, rx) = mpsc::channel(1024);
        let _ = JOB_QUEUE.set(tx);
        Self { callbacks: rx }
    }
}

pub(crate) static JOB_QUEUE: RunTimeLocal<OnceCell<Sender<Callback>>> = RunTimeLocal {
    __data: OnceCell::new(),
};

pub async fn dispatch(job: impl FnOnce(&mut EditorModel, &mut Compositor) + Send + 'static) {
    let _ = JOB_QUEUE.wait().send(Box::new(job)).await;
}

pub struct RunTimeLocal<T: 'static> {
    pub __data: T,
}

impl<T> Deref for RunTimeLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.__data
    }
}
