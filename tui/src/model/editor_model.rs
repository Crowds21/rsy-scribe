use crate::model::document_model::{DocumentId, DocumentModel};
use std::collections::BTreeMap;

/// 文档展示区的整体状态
pub struct EditorModel {
    /// 下一个新建的文档 id
    pub next_document_id: DocumentId,
    ///
    pub documents: BTreeMap<DocumentId, DocumentModel>,
}

impl Default for EditorModel {
    fn default() -> Self {
        EditorModel {
            next_document_id: DocumentId::default(),
            documents: BTreeMap::new(),
        }
    }
}
