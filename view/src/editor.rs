use crate::document::{DocumentId, DocumentModel};
use ratatui::prelude::Rect;
use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use syservice::lute::node::Node;

/// 文档展示区的整体状态
#[derive(Default)]
pub struct EditorModel {
    /// 下一个新建的文档 id
    pub next_document_id: DocumentId,
    /// TODO 当前文档 ID,后续可参考 helix 替换为 view 组件
    pub current_id: Option<DocumentId>,
    pub documents: BTreeMap<DocumentId, DocumentModel>,
    /// 编辑器的可展示区域
    pub area: Rect,
}
impl EditorModel {
    /// Create new document by given SY Node. Return a documentId which has not used yet.
    ///
    pub fn new_document(&mut self, node: Node,available_width:u16) -> DocumentId {
        let id = self.next_document_id;
        // Safety: adding 1 from 1 is fine, probably impossible to reach usize max
        self.next_document_id =
            DocumentId(unsafe { NonZeroUsize::new_unchecked(self.next_document_id.0.get() + 1) });
        
        // TODO 
        // self.area;
        let doc = DocumentModel::open(node, id, available_width);
        self.documents.insert(id, doc);
        self.current_id = Some(id);
        id
    }
    /// 获取当当前展示的文档.
    pub fn get_current_mutable_document(&mut self) -> Option<&mut DocumentModel> {
        if let Some(id) = &self.current_id {
            return self.documents.get_mut(id);
        };
        None
    }
}
