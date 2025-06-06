use crate::component::search_box::{SearchBox, SearchResultItem};
use crate::compositor::Compositor;
use crate::debounce::AsyncHook;
use crate::job::dispatch;
use std::time::Duration;
use syservice::document;
use tokio::task::JoinHandle;
use tokio::time::Instant;

const SEARCH_BOX_DEFAULT_DEBOUNCE: Duration = Duration::from_millis(500);
/// 尾防抖
pub struct SearchBoxDebounce {
    current_task: Option<JoinHandle<()>>, // 当前搜索任务句柄
    last_query: String,                   // 当前查询内容
    debounce: Duration,
}

impl SearchBoxDebounce {
    pub fn new() -> Self {
        Self {
            debounce: SEARCH_BOX_DEFAULT_DEBOUNCE,
            last_query: "".into(),
            current_task: None,
        }
    }
}
impl AsyncHook for SearchBoxDebounce {
    type Event = String;

    /// 如果不开启新的计时,返回 None
    fn handle_event(
        &mut self,
        input: Self::Event,
        timeout: Option<tokio::time::Instant>,
    ) -> Option<Instant> {
        if self.last_query == *input {
            None
        } else {
            self.last_query = input;
            Some(Instant::now() + SEARCH_BOX_DEFAULT_DEBOUNCE)
        }
    }

    /// 防抖结束时,发起接口调用. 接口返回后返回 UI 更新.
    fn finish_debounce(&mut self) {
        let query = self.last_query.clone();

        // TODO finish_debounce 的调用最好也是放入searchBox 中, debounce 本身
        //  只保留异步调用相关逻辑内容
        tokio::spawn(async move {
            let sy_blocks = document::search_doc_with_title(query).await;

            let update_search_result = move |compositor: &mut Compositor| {
                let component = compositor.find::<SearchBox>();
                if let Some(search_box) = component {
                    if let Ok(resp) = sy_blocks {
                        search_box.results = resp
                            .data
                            .iter()
                            .map(move |it| SearchResultItem {
                                id: it.id.clone(),
                                box_id: it.box_id.clone(),
                                content: it.content.clone(),
                                path: it.path.clone(),
                                hpath: it.hpath.clone(),
                            })
                            .collect();
                        search_box.selected_result = None;
                    }
                }
            };
            dispatch(update_search_result).await
        });
    }
}
