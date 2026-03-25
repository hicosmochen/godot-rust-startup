use godot::prelude::*;
use godot::classes::{RichTextLabel, IRichTextLabel}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=RichTextLabel)] 
pub struct ProjectRichTextLabel {
    base: Base<RichTextLabel>
}

#[godot_api]
impl IRichTextLabel for ProjectRichTextLabel {
    fn init(base: Base<RichTextLabel>) -> Self {
        godot_print!("文本域"); 
        Self {base}
    }

    fn ready(&mut self) {
        // 在 ready 中将信号连接到自身函数
        self.base_mut().add_to_group("log_receivers");
    }
}


// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectRichTextLabel {
    // 定义槽函数, 用于接收信号并且追加文本
    // 注意: 如果是当前对象发给自己的信号, 直接调用此方法即可
    #[func]
    pub fn on_add_log(&mut self, text: GString){
        let _text_str = Into::<String>::into(&text);
        // 直接使用 GString，它实现了向 RichTextLabel 追加所需的 trait
        let mut base = self.base_mut();
        base.append_text(&_text_str);
        base.append_text("\n");
    }
}



/*
    我可以采用下面的代码, 发送信号, 显示内容:
    寻找 "log_receivers" 组中的所有节点，并调用它们的 "on_add_log" 方法

    let message = format!("工作路径: {path}");


    self.base().get_tree().unwrap().call_group(
        "log_receivers", 
        "on_add_log", 
        &[message.to_variant()]
    );
*/