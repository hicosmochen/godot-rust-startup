use godot::prelude::*;
use godot::classes:: {FileDialog, IFileDialog};
use crate::secure::secure_storage::SecureStorage;

#[derive(GodotClass)]
#[class(base=FileDialog)]
pub struct MyFileDialog{
    pub kind: String,  // 必须在这里定义成员变量
    pub base: Base<FileDialog>
}

#[godot_api]
impl IFileDialog for MyFileDialog{
    fn init(base: Base<FileDialog>) -> Self{
        Self{ 
            base,
            kind: "default".to_string(), // 2. 初始化默认值
         }
    }
    // 当用户选择文件并且点击确定时触发
    fn ready(&mut self) {
        let on_file_selected_callable = self.base().callable("on_file_selected");
        let on_dir_selected_callable = self.base().callable("on_dir_selected");
        self.base_mut().connect("file_selected", &on_file_selected_callable);
        self.base_mut().connect("dir_selected", &on_dir_selected_callable);
    }
}

#[godot_api]
impl MyFileDialog {
    #[func]
    fn on_file_selected(&self, path: String){
        // 这里需要将内容缓存起来
        SecureStorage::save("path_godot", &path);
       godot_print!("用户选择文件: {}  类型: {}", path, self.kind);
    }

    #[func]
    fn on_dir_selected(&self, path: String){
        // 这里需要将内容缓存起来
        SecureStorage::save("path_rust", &path);
        godot_print!("用户选择文件: {}  类型: {}", path, self.kind);
    }

    
    #[func]
    pub fn open_dialog(&mut self){
        self.base_mut().popup_centered();
    }
}