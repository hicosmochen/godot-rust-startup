use godot::prelude::*;
use godot::classes::{Label, ILabel, Theme}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=Label)] 
pub struct AboutAuthorText {
    base: Base<Label>
}


#[godot_api]
impl ILabel for AboutAuthorText {
    fn init(base: Base<Label>) -> Self {
        godot_print!("联系作者"); 
        Self {base}
    }

    fn ready(&mut self) {
        let email = self.get_email();
        let mut node = self.base_mut();
        let email_message = format!("邮箱地址: {}", email);
        node.set_text(&email_message);

        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_body_label_theme.tres");
        // 应用到主节点，子节点会继承该主题
        node.set_theme(&my_theme);
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl AboutAuthorText {
    #[func]
    fn get_email(&self) -> String {
        "chcsvip@126.com".to_string()
    }
}