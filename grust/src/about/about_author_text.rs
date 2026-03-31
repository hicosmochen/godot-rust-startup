use godot::prelude::*;
use godot::classes::{Label, ILabel, Theme}; // 导入需要的 UI 类

use crate::secure::secure_storage::SecureStorage;

#[derive(GodotClass)]
#[class(base=Label)] 
pub struct AboutAuthorText {
    base: Base<Label>,
    text: GString,
}


#[godot_api]
impl ILabel for AboutAuthorText {
    fn init(base: Base<Label>) -> Self {
        godot_print!("联系作者"); 
        Self {
            base,
            text: GString::from("en-US"),
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group("listener_change_language");
        self.text = GString::from(&SecureStorage::get("current_lanague"));

        let email = self.get_email(self.text.clone());
        let mut node = self.base_mut();
        let translated_text_email_address = node.tr("email_address");
        let email_message = format!("{}: {}", translated_text_email_address.to_string(), email);
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

    // 改变语言
    #[func]
    fn on_change_language(&mut self, _text: GString){
        self.text = _text.clone();
        let email = self.get_email(self.text.clone());
        let mut node = self.base_mut();
        
        let translated_text_email_address = node.tr("email_address");
        let email_message = format!("{}: {}", translated_text_email_address.to_string(), email);
        node.set_text(&email_message);
    }

    #[func]
    fn get_email(&self, _text: GString) -> String {
        "chcsvip@126.com".to_string()
    }
}