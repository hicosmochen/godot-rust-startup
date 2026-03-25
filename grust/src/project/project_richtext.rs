use godot::prelude::*;
use godot::classes::{RichTextLabel, IRichTextLabel, Theme}; // 导入需要的 UI 类

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
        
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectRichTextLabel {

}