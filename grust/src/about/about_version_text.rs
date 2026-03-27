use godot::prelude::*;
use godot::classes::{Label, ILabel, Theme}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=Label)] 
pub struct AboutVersionText {
    base: Base<Label>
}


#[godot_api]
impl ILabel for AboutVersionText {
    fn init(base: Base<Label>) -> Self {
        godot_print!("版本信息"); 
        Self {base}
    }

    fn ready(&mut self) {
        let version = self.get_version();
        let mut node = self.base_mut();
        let version_message = format!("当前版本: {}", version);
        node.set_text(&version_message);

        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_body_label_theme.tres");
        // 应用到主节点，子节点会继承该主题
        node.set_theme(&my_theme);
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl AboutVersionText {
    #[func]
    fn get_version(&self) -> String {
        "2026.0327.1602".to_string()
    }
}