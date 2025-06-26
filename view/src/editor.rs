use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use crate::document::{DocumentId, DocumentModel};

/// 文档展示区的整体状态
#[derive(Default)]
pub struct EditorModel {
    /// 下一个新建的文档 id
    pub next_document_id: DocumentId,
    pub documents: BTreeMap<DocumentId, DocumentModel>,
}
impl EditorModel {
    fn new_document(&mut self, mut doc: DocumentModel) -> DocumentId {
        let id = self.next_document_id;
        // Safety: adding 1 from 1 is fine, probably impossible to reach usize max
        self.next_document_id =
            DocumentId(unsafe { NonZeroUsize::new_unchecked(self.next_document_id.0.get() + 1) });
        doc.id = id;
        self.documents.insert(id, doc);
        
        // let (save_sender, save_receiver) = tokio::sync::mpsc::unbounded_channel();
        // self.saves.insert(id, save_sender);
        // 
        // let stream = UnboundedReceiverStream::new(save_receiver).flatten();
        // self.save_queue.push(stream);
        id
    }
}