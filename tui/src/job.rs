use std::future::Future;
use crate::compositor::Compositor;
use once_cell::sync::OnceCell;
use std::ops::Deref;
use std::sync::OnceLock;
use tokio::sync::mpsc::{self, Receiver, Sender};
use crate::model::editor_model::EditorModel;

type Callback = Box<dyn FnOnce(&mut Compositor) + Send + 'static>;

/// 全局任务队列
pub struct JobQueue {
    // pub tx: Sender<Callback>,
    pub callbacks: Receiver<Callback>,
}

impl JobQueue {
    pub fn handle_callback(
        &self,
        editor_model:&mut EditorModel,
        compositor: &mut Compositor,
        call: anyhow::Result<Option<Callback>>,
    ) {
        match call {
            Ok(None) => {}
            Ok(Some(call)) => call(compositor),
            Err(e) => {
                
            }
        }
    }

    pub fn new() -> Self {
        static INSTANCE: OnceLock<JobQueue> = OnceLock::new();
        let (tx, rx) = mpsc::channel(1024);
        let _ = JOB_QUEUE.set(tx);
        Self { callbacks: rx }
    }

    pub fn handle_callbacks(compositor: &mut Compositor) {
        todo!()
    }
    fn process_callbacks(mut rx: Receiver<Callback>, compositor: &mut Compositor) {
        // while let Some(callback) = rx.recv().await {
        //     callback(compositor); // 执行回调
        // }
        todo!()
    }
}

pub(crate) static JOB_QUEUE: RunTimeLocal<OnceCell<Sender<Callback>>> = {
    RunTimeLocal {
        __data: (OnceCell::new()),
    }
};

pub async fn dispatch(job: impl FnOnce(&mut Compositor) + Send + 'static) {
    let _ = JOB_QUEUE.wait().send(Box::new(job)).await;
}
pub async fn dispatch_(job: impl FnOnce(&mut Compositor) + 'static) {
    
}

///
// pub fn dispatch_blocking(job: impl FnOnce(&mut Compositor) + Send + 'static) {
//     let jobs = JOB_QUEUE
//         .wait().blocking_send(Box::new(job));
// }
pub struct RunTimeLocal<T: 'static> {
    pub __data: T,
}

impl<T> Deref for RunTimeLocal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.__data
    }
}
