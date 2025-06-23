use tui::component::block::{doc, RenderedBlock};
use tui::compositor::CompositorContext;

#[test]
fn parse_sy_file_test() {
    let sy_file = syservice::test_utils::load_sy_test_file();
    assert!(sy_file.is_ok());
    let mut root = sy_file.unwrap();
    let cx = CompositorContext::new();
    let mut vec: Vec<RenderedBlock> = Vec::new();
    root.set_node_type_for_tree();
    doc::create_tui_element(&root, &cx, &mut vec);
    assert!(!vec.is_empty())
}
