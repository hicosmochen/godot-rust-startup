use godot::prelude::*;
use godot::classes:: {FileDialog, IFileDialog};
use crate::secure::secure_storage::SecureStorage;
use godot::classes::ConfigFile;  // 正确导入 配置文件信息
use godot::global::Error;        // 正确导入 Godot 的全局错误枚举

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
            kind: String::from("default"), // 2. 初始化默认值
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
    // 文件的选择
    #[func]
    fn on_file_selected(&mut self, path: String){
        match self.kind.as_str() {
             "path_godot" => {
                SecureStorage::save("path_godot", &path);
                godot_print!("用户选择文件: {}  类型: {}", path, self.kind);
            },
            "work_space" => {
                godot_print!("用户选择文件: {}  类型: {}", path, self.kind);
            },
            &_ => {
                godot_print!("用户选择其他文件");
            },
        };
    }

    // 文件夹的选择
    #[func]
    fn on_dir_selected(&mut self, path: String){
        match self.kind.as_str() {
            "path_rust" => {
                SecureStorage::save("path_rust", &path);
                godot_print!("用户选择文件夹: {}  类型: {}", path, self.kind);
            },
            "work_space" => {
                godot_print!("用户选择文件夹: {}  类型: {}", path, self.kind);
                //----------------------------------------------------------------
                // 构建配置对象
                let mut config = ConfigFile::new_gd();
                // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
                let _ = config.load("res://config.cfg"); 
                config.set_value("Editor", "work_space", &path.to_variant());
                // 保存并检查结果
                let result = config.save("res://config.cfg");
                if result == Error::OK {
                    godot_print!("路径已成功保存: {}", path);
                    self.send_message_to_rich(format!("工作路径: {path}"));
                } else {
                    godot_print!("存储失败: {:?}", result);
                }
                //----------------------------------------------------------------
            },
            &_ => {
                 godot_print!("用户选择其他文件夹");
            },
        };
    }


    // 发送信息给 富文本显示内容
    #[func]
    pub fn send_message_to_rich(&mut self, message: String){
        self.base().get_tree().unwrap().call_group(
                        "log_receivers", 
                        "on_add_log", 
                        &[message.to_variant()]
                    );
    }

    
    #[func]
    pub fn open_dialog(&mut self){
        self.base_mut().popup_centered();
    }
}