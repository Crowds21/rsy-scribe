use std::collections::{HashMap, HashSet};
use syservice::lute::node::{Node, NodeType};

// pub struct VirtualViewport {
//     /// 真实视窗高度（行数）
//     pub real_height: usize,
//     /// 扩展后的渲染高度（real_height * 2）
//     pub render_height: usize,
//     /// 当前滚动位置
//     pub scroll_pos: usize,
//     /// 总内容高度
//     pub total_height: usize,
//     /// 渲染起始偏移（动态计算）
//     pub render_offset: usize,
// }
// 
// impl VirtualViewport {
//     pub fn new(real_height: usize) -> Self {
//         Self {
//             real_height,
//             render_height: real_height * 2,
//             scroll_pos: 0,
//             total_height: 0,
//             render_offset: 0,
//         }
//     }
// 
//     /// 更新滚动位置并计算渲染范围
//     pub fn update_scroll(&mut self, new_pos: usize) {
//         self.scroll_pos = new_pos.min(self.total_height.saturating_sub(self.real_height));
// 
//         // 计算200%渲染区域的起始位置
//         let ideal_offset = self.scroll_pos.saturating_sub(self.real_height / 2);
//         self.render_offset = ideal_offset.min(
//             self.total_height.saturating_sub(self.render_height)
//         );
//     }
// 
//     /// 获取实际需要渲染的范围
//     pub fn render_range(&self) -> (usize, usize) {
//         let start = self.render_offset;
//         let end = (start + self.render_height).min(self.total_height);
//         (start, end)
//     }
// 
//     /// 检查某个节点是否在扩展渲染区内
//     pub fn should_render(&self, node_start: usize, node_height: usize) -> bool {
//         let node_end = node_start + node_height;
//         let (render_start, render_end) = self.render_range();
//         node_end > render_start && node_start < render_end
//     }
// }
// pub struct NodePositionTracker {
//     /// 节点ID到位置信息的映射
//     positions: HashMap<usize, NodePosition>,
//     /// 按位置排序的节点ID列表
//     sorted_nodes: Vec<usize>,
// }
// 
// #[derive(Debug)]
// struct NodePosition {
//     start_line: usize,
//     height: usize,
// }
// 
// impl NodePositionTracker {
//     pub fn build(root: &Node) -> Self {
//         let mut tracker = Self {
//             positions: HashMap::new(),
//             sorted_nodes: Vec::new(),
//         };
//         tracker.measure_nodes(root, 0);
//         tracker
//     }
// 
//     fn measure_nodes(&mut self, node: &Node, mut current_pos: usize) -> usize {
//         let node_id = node.id.unwrap_or_default(); // 假设有唯一ID
// 
//         let height = match node.node_type {
//             NodeType::NodeParagraph=> calculate_paragraph_height(&node.content),
//             _ => 1,
//         };
// 
//         self.positions.insert(node_id, NodePosition {
//             start_line: current_pos,
//             height,
//         });
//         self.sorted_nodes.push(node_id);
// 
//         current_pos += height;
// 
//         for child in &node.children {
//             current_pos = self.measure_nodes(child, current_pos);
//         }
// 
//         current_pos
//     }
// }
// pub struct SmartRenderer {
//     viewport: VirtualViewport,
//     tracker: NodePositionTracker,
//     /// 当前已加载的节点
//     loaded_nodes: HashSet<usize>,
//     /// 需要卸载的节点
//     pending_unload: Vec<usize>,
// }
// 
// impl SmartRenderer {
//     pub fn new(root: &Node, initial_height: usize) -> Self {
//         let tracker = NodePositionTracker::build(root);
//         let total_height = tracker.positions.values()
//             .map(|p| p.height)
//             .sum();
// 
//         Self {
//             viewport: VirtualViewport::new(initial_height).with_total_height(total_height),
//             tracker,
//             loaded_nodes: HashSet::new(),
//             pending_unload: Vec::new(),
//         }
//     }
// 
//     /// 主更新方法
//     pub fn update_viewport(&mut self, new_scroll_pos: usize) -> RenderUpdate {
//         self.viewport.update_scroll(new_scroll_pos);
// 
//         let (load, unload) = self.calculate_changes();
// 
//         RenderUpdate {
//             nodes_to_load: load,
//             nodes_to_unload: unload,
//             visual_offset: self.viewport.render_offset,
//         }
//     }
// 
//     fn calculate_changes(&mut self) -> (Vec<usize>, Vec<usize>) {
//         let mut to_load = Vec::new();
//         let mut to_unload = Vec::new();
// 
//         // 检查当前应渲染的节点
//         for &node_id in &self.tracker.sorted_nodes {
//             let pos = &self.tracker.positions[&node_id];
//             let should_render = self.viewport.should_render(pos.start_line, pos.height);
//             let is_loaded = self.loaded_nodes.contains(&node_id);
// 
//             match (should_render, is_loaded) {
//                 (true, false) => to_load.push(node_id),
//                 (false, true) => to_unload.push(node_id),
//                 _ => {}
//             }
//         }
// 
//         (to_load, to_unload)
//     }
// }