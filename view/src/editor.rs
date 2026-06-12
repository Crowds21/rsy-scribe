use crate::document::{DocumentId, DocumentModel};
use ratatui::prelude::Rect;
use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use syservice::lute::node::Node;
use crate::view::ViewPosition;

/// 文档展示区的整体状态
#[derive(Default)]
pub struct EditorModel {
    /// 下一个新建的文档 id
    pub next_document_id: DocumentId,
    /// TODO 当前文档 ID,后续可参考 helix 替换为 view 组件
    pub current_id: Option<DocumentId>,
    pub documents: BTreeMap<DocumentId, DocumentModel>,
    /// TODO 保存不同的文档的偏移量,因为整个关闭文档后,下次重新打开可能还希望保持在原位置
    ///  因此滑动信息不能随文档的打开和关闭而丢失
    ///  包括上下滑动,水平滑动.
    view_position: ViewPosition
}
impl EditorModel {
    /// Create new document by given SY Node. Return a documentId which has not used yet.
    ///
    pub fn new_document(
        &mut self,
        node: Node,
        available_width: u16,
        source_label: String,
    ) -> DocumentId {
        let id = self.next_document_id;
        // Safety: adding 1 from 1 is fine, probably impossible to reach usize max
        self.next_document_id =
            DocumentId(unsafe { NonZeroUsize::new_unchecked(self.next_document_id.0.get() + 1) });

        let doc = DocumentModel::open(node, id, available_width, source_label);
        self.documents.insert(id, doc);
        self.current_id = Some(id);
        id
    }

    pub fn get_current_document(&self) -> Option<&DocumentModel> {
        self.current_id.and_then(|id| self.documents.get(&id))
    }

    /// 已打开文档的展示名，用于 buffer 栏
    pub fn open_document_labels(&self) -> Vec<(DocumentId, String)> {
        self.documents
            .iter()
            .map(|(id, doc)| (*id, doc.display_name()))
            .collect()
    }
    /// 所当前文档所需要占用的总行数
    pub fn get_current_doc_height(&self) -> u16 {
        if let Some(id) = self.current_id {
            return self.get_doc_height(&id);
        }
        0
    }

    /// 获取指定 `document_id` 所需要占用的总行数
    pub fn get_doc_height(&self,document_id: &DocumentId) -> u16 {
        let doc_model = self.documents.get(document_id);
        if let Some(doc) = doc_model {
            return doc.lines.len() as u16;
        }
        0
    }
    /// 获取当当前展示的文档.
    pub fn get_current_mutable_document(&mut self) -> Option<&mut DocumentModel> {
        if let Some(id) = &self.current_id {
            return self.documents.get_mut(id);
        };
        None
    }
}
