use godot::prelude::*;
use godot::classes::{Label, ILabel, Theme}; // 导入需要的 UI 类

use crate::secure::secure_storage::SecureStorage;

#[derive(GodotClass)]
#[class(base=Label)] 
pub struct AboutVersionText {
    base: Base<Label>,
    text: GString,
}


#[godot_api]
impl ILabel for AboutVersionText {
    fn init(base: Base<Label>) -> Self {
        godot_print!("版本信息"); 
        Self {
            base,
            text: GString::from("en-US"),
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group("listener_change_language");
        self.text = GString::from(&SecureStorage::get("current_lanague"));


        let version = self.get_version(self.text.clone());
        let mut node = self.base_mut();
        let translated_text_current_version = node.tr("current_version");
        let version_message = format!("{}: {}", translated_text_current_version.to_string(), version);
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

    // 改变语言
    #[func]
    fn on_change_language(&mut self, _text: GString){
        self.text = _text.clone();
        let version = self.get_version(self.text.clone());
        let mut node = self.base_mut();
        
        let translated_text_current_version = node.tr("current_version");
        let version_message = format!("{}: {}", translated_text_current_version.to_string(), version);
        node.set_text(&version_message);
    }


    #[func]
    fn get_version(&mut self, _text: GString) -> String {
        "2026.0331.1740".to_string()
    }
}