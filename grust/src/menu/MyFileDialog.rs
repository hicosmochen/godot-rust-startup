use godot::prelude::*;
use godot::classes:: {FileDialog, IFileDialog};

#[derive(GodotClass)]
#[class(base=FileDialog)]
pub struct MyFileDialog{
    pub base: Base<FileDialog>
}

#[godot_api]
impl IFileDialog for MyFileDialog{
    fn init(base: Base<FileDialog>) -> Self{
        Self{ base }
    }
    // 当用户选择文件并且点击确定时触发
    fn ready(&mut self) {
        let callable = self.base().callable("on_file_selected");
        self.base_mut().connect("file_selected", &callable);
    }
}

#[godot_api]
impl MyFileDialog {
    #[func]
    fn on_file_selected(&self, path: String){
        godot_print!("用户选择文件: {}", path);
    }

    #[func]
    pub fn open_dialog(&mut self){
        self.base_mut().popup_centered();
    }
}