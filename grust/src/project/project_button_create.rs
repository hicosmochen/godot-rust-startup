use godot::prelude::*;
use godot::classes::{Button, IButton}; // 导入需要的 UI 类

use godot::classes::ConfigFile;  // 正确导入 配置文件信息

use std::sync::mpsc;
use std::os::windows::process::CommandExt; // 必须导入

use crate::project::project_richtext::ProjectRichTextLabel;
use crate::secure::secure_storage::SecureStorage;


#[derive(GodotClass)]
#[class(base=Button)] 
pub struct ProjectButtonCreate {
    base: Base<Button>,
    // 外部加入显示区域的 富文本信息
    #[export]
    pub label_rich: Option<Gd<ProjectRichTextLabel>>,

    // 使用 Option 包装，因为在 init 时还没有通道
    receiver: Option<mpsc::Receiver<String>>,

    path_rust  : String,
    path_godot : String,
    work_space : String,
    rust_root  : String,
    gdext_name : String,
    create_demo : bool,
}


#[godot_api]
impl IButton for ProjectButtonCreate {
    fn init(base: Base<Button>) -> Self {
        godot_print!("创建项目按钮"); 
        Self {
            base,
            label_rich: None, // 初始化必须为 None，等待 Godot 注入,
            receiver : None,
            path_rust : "".to_string(),
            path_godot : "".to_string(),
            work_space : "".to_string(),
            rust_root : "".to_string(),
            gdext_name : "".to_string(),
            create_demo : false,
        }
    }

    fn ready(&mut self) {
        // 绑定信号
        let button_pressed =  self.base().callable("on_button_pressed");
        self.base_mut().connect("pressed", &button_pressed);
    }


    fn process(&mut self, _delta: f64) {
        let mut messages = Vec::new();
        // 1. 快速检查通道，仅不可变借用 self.receiver
        if let Some(ref rx) = self.receiver {
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }
        // 2. 此时不可变借用已结束，可以安全地进行可变借用  CARGO_ERROR
        for msg in messages {
            if msg.to_string() == "CARGO_SUCCESS"{
                godot_print!("Cargo 创建完毕了xxx");
                self.send_message_to_rich(format!("Cargo 创建完毕了"));
                self.cargo_add_godot();
            }else if msg.to_string() == "ADD_GODOT_SUCCESS" {
                godot_print!("add godot 创建完毕了xxx");
                self.send_message_to_rich(format!("add godot 创建完毕了"));
                self.modify_cargo_toml();
            }else if msg.to_string() == "CARGO_ERROR" {
                godot_print!("add godot 创建失败了xxx");
                self.send_message_to_rich(format!("add godot 创建完毕了"));
            }
        }
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectButtonCreate {

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
    fn on_button_pressed(&mut self){
        if self.start_create_project(){
            godot_print!("开始创建项目");
            self.creat_rust_project();
        }else{
            godot_print!("条件不满足");
        }
    }


    
    /*
        开始创建项目
            1、 创建 rust 项目
            2、 修改 cargo 文件 ---------------------------- 自动化实现 （配置文件的修改） name = "myrust" # rust 所在的文件夹名称
            3、 添加 godot 关联 rust ----------------------- 自动化实现 （执行命令） cargo  add  godot
            4、 创建 godot 项目 ---------------------------- 自动化实现
            5、 创建 gdextension 文件路径 和 文件名称 -------- 用户选择
            6、 检测映射文件 extension_list.cfg  ----------- 自动化实现
    */
    #[func]
    pub fn start_create_project(&mut self) -> bool{
        godot_print!("按钮被点击了..start_create_project.StartCreatProject");
        // 获取到前面存储的所有数据
        // 1、Godot 启动文件
        self.path_godot = SecureStorage::get("path_godot");
        self.path_rust = SecureStorage::get("path_rust");
        // 构建配置对象
        let mut config = ConfigFile::new_gd();
        // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
        let _ = config.load("res://config.cfg"); 
        // 带默认值的安全获取（推荐方式）
        self.work_space = config
            .get_value_ex("Editor", "work_space")
            .default(&"".to_variant())  
            .done()
            .try_to::<String>()
            .unwrap_or_default(); // 如果失败或不存在，返回空字符串 ""

        self.rust_root = config
            .get_value_ex("Editor", "rust_root")
            .default(&"".to_variant())  
            .done()
            .try_to::<String>()
            .unwrap_or_default(); // 如果失败或不存在，返回空字符串 ""

        self.gdext_name = config
            .get_value_ex("Editor", "gdext_name")
            .default(&"".to_variant())  
            .done()
            .try_to::<String>()
            .unwrap_or_default(); // 如果失败或不存在，返回空字符串 ""

        self.create_demo = config
            .get_value_ex("Editor", "create_demo")
            .default(&false.to_variant())  
            .done()
            .to::<bool>();

        if self.path_godot.is_empty(){
            self.send_message_to_rich(format!("Godot 的路径不能为空"));
            return false;
        }
        
        if self.path_rust.is_empty(){
            self.send_message_to_rich(format!("Rust 的路径不能为空"));
            return false;
        }
        
        if self.work_space.is_empty(){
            self.send_message_to_rich(format!("工作空间 的路径不能为空"));
             return false;
        }
        
        if self.gdext_name.is_empty(){
            self.send_message_to_rich(format!("gdext的名称不能为空"));
             return false;
        }
         return true;
    }


    // 创建 rust 项目
    #[func]
    fn creat_rust_project(&mut self) {
        let cargo_path =  format!("{}/bin/cargo.exe", self.path_rust);
        self.send_message_to_rich(format!("cargo: {cargo_path}"));

        // 克隆变量以进入线程闭包
        let rust_root = self.rust_root.clone();
        let work_space_clone = self.work_space.clone();

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx); // 将接收端交给主线程轮询
        std::thread::spawn(move || {
            // 1. 发送开始信号
            let _ = tx.send("任务开始...".to_string());
            // Windows 隐藏窗口标志位
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // 2. 执行耗时操作
            let mut binding = std::process::Command::new(cargo_path);
            let cmd = binding.arg("new")
                .arg(rust_root)
                .arg("--lib")
                .current_dir(work_space_clone);
            // 仅在 Windows 下配置隐藏窗口
            #[cfg(windows)]
            {
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            let output = cmd.output();

            // 3. 发送结果
            match output {
                Ok(_) => { let _ = tx.send("CARGO_SUCCESS".to_string()); }
                Err(e) => { let _ = tx.send(format!("CARGO_FAIL: {}", e)); }
            }
        });
    }


    // 自动化实现 （执行命令） cargo  add  godot
    #[func]
    fn cargo_add_godot(&mut self){
        let cargo_path =  format!("{}/bin/cargo.exe", self.path_rust);
        // 需要执行下面的命令
        let work_space_clone =  format!("{}/{}", self.work_space, self.rust_root);

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx); // 将接收端交给主线程轮询
        std::thread::spawn(move || {
            // 1. 发送开始信号
            let _ = tx.send("任务开始...".to_string());

            // Windows 隐藏窗口标志位
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // 2. 执行耗时操作
            let mut binding = std::process::Command::new(cargo_path);
            let cmd = binding.arg("add") 
                .arg("godot")
                .current_dir(work_space_clone);

            // 仅在 Windows 下配置隐藏窗口
            #[cfg(windows)]
            {
                cmd.creation_flags(CREATE_NO_WINDOW);
            }

            let output = cmd.output();

            // 3. 发送结果
            match output {
               Ok(out) => {
                    if out.status.success() {
                        let _ = tx.send("ADD_GODOT_SUCCESS".to_string());
                    } else {
                        // 关键修正 2: 捕获并发送真正的错误信息
                        let error_msg = String::from_utf8_lossy(&out.stderr);
                        let _ = tx.send(format!("CARGO_ERROR: {}", error_msg));
                    }
                }
                Err(e) => {
                    let _ = tx.send(format!("PROCESS_FAIL: {}", e));
                }
            }
        });
    }


    // 修改 cargo toml 的名称
    #[func]
    fn modify_cargo_toml(&mut self) {
        let cargo_toml_path = format!("{}/{}/Cargo.toml", self.work_space, self.rust_root);

        // 封装逻辑以使用 ? 语法
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            let content = std::fs::read_to_string(&cargo_toml_path)?;
            let mut doc = content.parse::<toml_edit::DocumentMut>()?;

            // 错误 2 修复：toml_edit 的 value() 不支持直接从 Vec<&str> 转换
            // 需要显式转换为 Value::from_iter
            let mut lib_table = toml_edit::Table::new();
            let crate_types: toml_edit::Array = vec!["cdylib"].into_iter().collect();
            lib_table["crate-type"] = toml_edit::value(crate_types);
            
            doc.insert("lib", toml_edit::Item::Table(lib_table));

            std::fs::write(&cargo_toml_path, doc.to_string())?;
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("modify cargo toml 创建完毕了");
                self.send_message_to_rich(format!("modify cargo toml 创建完毕了"));
                self.modify_lib_rs();
            }
            Err(_e) => {
                godot_print!("modify cargo toml 创建失败了");
                self.send_message_to_rich(format!("modify cargo toml 创建失败了"));
            }
        }
    }

    // 需要创建 lib.rs 文件, 并且加入必须的数据
    #[func]
    fn modify_lib_rs(&mut self) {
        // 定义需要操作的路径
        let lib_rs_path = format!("{}/{}/src/lib.rs", self.work_space, self.rust_root);
        // 定义需要操作的文本内容
        let content = r#"use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

}"#;

        // 封装逻辑以使用 ? 语法
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            std::fs::write(&lib_rs_path, content.to_string())?;
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("modify lib rs 创建完毕了");
                self.send_message_to_rich(format!("modify lib rs 创建完毕了"));
                self.create_godot_project();
            }
            Err(_e) => {
                godot_print!("modify lib rs  创建失败了");
                self.send_message_to_rich(format!("modify lib rs  创建失败了"));
            }
        }
    }


    /*
        1、 创建 godot 的文件夹
        2、 在文件夹当中, 需要创建文件 project.godot
        3、 需要创建文件 my_game.gdextension 文件中写入数据 
        4、 往 my_game.gdextension 文件中写入配置数据
        5、 启动 godot --editor
        6、 需要修改配置文件的路径信息:  C:\Users\Administrator\AppData\Roaming\Godot\projects.cfg
    */
    #[func]
    fn create_godot_project(&mut self) {
        godot_print!("创建 godot 的文件夹");
    }
}