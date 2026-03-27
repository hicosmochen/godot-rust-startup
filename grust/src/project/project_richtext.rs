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
         let text_str = text.to_string();
        // 根据条件选择颜色代码
        let color_code =  "black";
        // 构造 BBCode 字符串: [color=red]内容[/color]
        let formatted_text = format!("[color={}]{}[/color]\n", color_code, text_str);
        let mut base = self.base_mut();
        base.append_text(&formatted_text);
    }


    // 失败
    #[func]
    pub fn on_add_log_fail(&mut self, text: GString) {
        let text_str = text.to_string();
        // 根据条件选择颜色代码
        let color_code =  "red";
        // 构造 BBCode 字符串: [color=red]内容[/color]
        let formatted_text = format!("[color={}]{}[/color]\n", color_code, text_str);
        let mut base = self.base_mut();
        base.append_text(&formatted_text);
    }


    // 成功
    #[func]
    pub fn on_add_log_success(&mut self, text: GString) {
          let text_str = text.to_string();
        // 根据条件选择颜色代码
        let color_code = "#038c03ff";
        // 构造 BBCode 字符串: [color=red]内容[/color]
        let formatted_text = format!("[color={}]{}[/color]\n", color_code, text_str);
        let mut base = self.base_mut();
        base.append_text(&formatted_text);
    }


    // 强调
    #[func]
    pub fn on_add_log_emphasize(&mut self, text: GString) {
          let text_str = text.to_string();
        // 根据条件选择颜色代码
        let color_code = "#3504e7ff";
        // 构造 BBCode 字符串: [color=red]内容[/color]
        let formatted_text = format!("[color={}]{}[/color]\n", color_code, text_str);
        let mut base = self.base_mut();
        base.append_text(&formatted_text);
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