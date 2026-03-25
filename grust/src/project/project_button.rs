use godot::prelude::*;
use godot::classes::{Button, IButton, Theme}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=Button)] 
pub struct ProjectButton {
    pub base: Base<Button>,
    pub kind: String
}


#[godot_api]
impl IButton for ProjectButton {
    fn init(base: Base<Button>) -> Self {
        godot_print!("按钮"); 
        Self {
            base,
            kind: "default".to_string(),   // 默认初始化
        }
    }

    fn ready(&mut self) {
        // 绑定信号
        let button_pressed =  self.base().callable("on_button_pressed");
        self.base_mut().connect("pressed", &button_pressed);
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectButton {
    #[func]
    fn on_button_pressed(){
         godot_print!("按钮被点击了..."); 
    }
}