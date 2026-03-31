use godot::prelude::*;
use godot::classes::{Node, INode}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=Node)] 
pub struct LanguageDialog {
    base: Base<Node>
}


#[godot_api]
impl INode for LanguageDialog {
    fn init(base: Base<Node>) -> Self {
        Self {base}
    }

    fn ready(&mut self) {
       // 将当前节点加入 "node_dialog" 组
        self.base_mut().add_to_group("node_dialog");
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl LanguageDialog {
     
}