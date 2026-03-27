use godot::prelude::*;
use godot::classes::{Button, IButton}; // 导入需要的 UI 类

use godot::classes::ConfigFile;  // 正确导入 配置文件信息

use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc;
use std::process::Command;
use std::os::windows::process::CommandExt; // 必须导入
use std::io::Write;
use std::env;
use std::path::PathBuf;

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
        // 2. 此时不可变借用已结束，可以安全地进行可变借用  CARGO_BUILD_PROJECT
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
            }else if msg.to_string() == "CARGO_BUILD_INIT" {
                godot_print!("cargo build init 创建完毕了");
                self.send_message_to_rich(format!("cargo build init 创建完毕了"));
                self.create_godot_project();
            }else if msg.to_string() == "GODOT_START_UP" {
                godot_print!("godot start up 创建完毕了");
                self.send_message_to_rich(format!("godot start up 创建完毕了"));
            }else if msg.to_string() == "CARGO_BUILD_PROJECT" {
                godot_print!("cargo build project 创建完毕了");
                self.send_message_to_rich(format!("cargo build project 创建完毕了"));
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
                self.cargo_build("CARGO_BUILD_INIT".to_string());
            }
            Err(_e) => {
                godot_print!("modify lib rs  创建失败了");
                self.send_message_to_rich(format!("modify lib rs  创建失败了"));
            }
        }
    }




    
    // 自动化实现 （执行命令） cargo  build
    #[func]
    fn cargo_build(&mut self, action: String){
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
            let cmd = binding.arg("build")
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
                        let _ = tx.send(action);
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





    #[func]
    fn create_godot_project(&mut self) {
        // 1. 使用 Path 处理路径，String 类型本身没有 .exists()
        let godot_project_path_str = format!("{}/godot", self.work_space);
        let godot_project_path = Path::new(&godot_project_path_str);

        // 2. 创建文件夹
        if !godot_project_path.exists() {
            // 在 #[func] 中不能直接用 ?，需处理 Result
            if let Err(e) = fs::create_dir_all(godot_project_path) {
                godot_print!("创建文件夹失败: {}", e);
                return;
            }
            self.send_message_to_rich("文件夹 godot 创建成功".to_string());
            godot_print!("文件夹 godot 创建成功");
        }

        // 3. 定义并执行创建文件逻辑
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            // 修正变量名：使用上面定义的 godot_project_path
            let project_file_path = godot_project_path.join("project.godot");
            fs::File::create(project_file_path)?; 
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("project.godot 创建成功");
                self.send_message_to_rich("project.godot 创建成功".to_string());
                self.create_file_gdextension();
            }
            Err(e) => {
                godot_print!("project.godot 创建失败了: {}", e);
                self.send_message_to_rich(format!("project.godot 创建失败: {}", e));
            }
        }
    }


    // 创建 gdextension 文件
    #[func]
    fn create_file_gdextension(&mut self) {
        godot_print!("需要创建文件 create file gdextension 文件中写入数据 ");
        // 定义需要操作的路径
        let file_gdextension_path = format!("{}/godot/{}.gdextension", self.work_space, self.gdext_name);
        // 定义需要操作的文本内容
        // 1. 定义你的变量
        let project_name = self.rust_root.clone(); 

        // 2. 使用 format! 宏进行拼接
        // 注意：如果字符串内部包含 { }（如 Godot 的 dict），需要用双大括号 {{ }} 转义，
        // 但在此配置文件格式中，直接使用 {} 即可。
        let content = format!(
    r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
windows.debug.x86_64 = "res://../{}/target/debug/{}.dll""#, 
            project_name, project_name
        );

        // 封装逻辑以使用 ? 语法
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            fs::File::create(&file_gdextension_path)?; // 注意加了 &
            std::fs::write(&file_gdextension_path, content)?; // 现在可以正常使用了
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("create file gdextension 创建完毕了");
                self.send_message_to_rich(format!("create file gdextension 创建完毕了"));
                // 准备修改配置文件
                self.modify_godot_projects();
            }
            Err(_e) => {
                godot_print!("create file gdextension  创建失败了");
                self.send_message_to_rich(format!("create file gdextension  创建失败了"));
            }
        }
    }

    

    // 修改godot 项目的配置文件信息, 让当前创建的项目, 在项目列表中展示
    // 路径 C:\Users\Administrator\AppData\Roaming\Godot\projects.cfg 
    #[func]
    fn modify_godot_projects(&mut self) {
        // 自动获取当前用户的 AppData/Roaming 路径
        let mut config_path = PathBuf::from(env::var("APPDATA").unwrap());
        config_path.push("Godot");
        config_path.push("projects.cfg");

        if config_path.exists() == false {
            godot_print!("未找到配置文件，请确认 Godot 是否已安装并在该用户下运行过。");
            return
        }

        // 封装逻辑以使用 ? 语法
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            let godot_project_path_str = format!("{}/godot", self.work_space);
            
            // 1. 先读取整个文件的内容
            let content = std::fs::read_to_string(&config_path)?;

            // 2. 检查路径是否已经存在（检查是否包含 "[路径]" 这种特征字符串）
            let section_header = format!("[{}]", godot_project_path_str);
            
            if content.contains(&section_header) {
                godot_print!("项目路径已存在，跳过追加。");
                return Ok(());
            }

            // 3. 如果不存在，再以追加模式打开并写入
            let mut file = OpenOptions::new().append(true).open(&config_path)?;
            
            // 确保新内容另起一行，并添加配置项
            let entry = format!(
                "\n{}\nfavorite=false\n", 
                section_header
            );
            
            file.write_all(entry.as_bytes())?;
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("modify godot projects 创建完毕了");
                self.send_message_to_rich(format!("modify godot projects 创建完毕了"));


                self.send_message_to_rich(format!("是否需要创建Demo案例: {}", self.create_demo));


                // 这里判断是否需要创建 Demo 程序?
                if self.create_demo {
                    self.append_mod_declaration();
                }else{
                    let mut scene_tree = self.base().get_tree().expect("无法获取 SceneTree");
                        // 关键点：在 create_timer 后面加上 .expect(...) 或 .unwrap()
                        scene_tree.create_timer(2.0)
                            .expect("无法创建计时器")
                            .connect(
                                "timeout", 
                                &self.base().callable("start_up_godot")
                        );
                }
            }
            Err(_e) => {
                godot_print!("modify godot projects  创建失败了");
                self.send_message_to_rich(format!("modify godot projects  创建失败了"));
            }
        }
    }




    // 需要创建 lib.rs 文件中追加内容
    #[func]
    fn append_mod_declaration(&mut self) {
        let librs_path =  format!("{}/{}/src/lib.rs", self.work_space, self.rust_root);

        let target_mod = "mod node_hello;"; // 注意：Rust 模块声明通常带分号

        // 1. 尝试读取现有内容
        // 如果文件不存在，这里会返回 Err，建议先做判断
        let content;
        if Path::new(&librs_path).exists() {
            match fs::read_to_string(&librs_path) {
                Ok(c) => content = c,
                Err(e) => {
                    godot_print!("读取 lib.rs 失败: {}", e);
                    return;
                }
            }
        } else {
            godot_print!("文件不存在: {}", librs_path);
            return;
        }

        // 2. 检查是否已经包含该模块声明
        // 使用 .contains 检查，可以避免重复添加
        if content.contains(target_mod) {
            godot_print!("lib.rs 已包含 {}，跳过修改。", target_mod);
            return;
        }

        // 3. 执行追加操作
        let execute_append = || -> Result<(), std::io::Error> {
            let mut file = OpenOptions::new()
                .append(true)
                .open(librs_path)?;

            // 确保新行开始，并在末尾添加换行符以保持代码整洁
            let entry = if content.ends_with('\n') {
                format!("{}\n", target_mod)
            } else {
                format!("\n{}\n", target_mod)
            };

            file.write_all(entry.as_bytes())?;
            Ok(())
        };

        match execute_append() {
            Ok(_) => {
                godot_print!("成功在 lib.rs 中添加模块声明");
                self.send_message_to_rich(format!("成功在 lib.rs 中添加模块声明"));
                self.create_file_node_hello();
            },
            Err(e) => {
                godot_print!("写入 lib.rs 失败: {}", e);
                self.send_message_to_rich(format!("写入 lib.rs 失败"));
            },
        }
    }


    // 创建 node_hello 文件
    #[func]
    fn create_file_node_hello(&mut self) {
        godot_print!("需要创建文件 create file node hello 文件中写入数据 ");
        // 定义需要操作的路径
        let file_node_hello_path = format!("{}/{}/src/node_hello.rs", self.work_space, self.rust_root);
  
        // 准备要写入的内容
        let content = r#"use godot::prelude::*;
use godot::classes::{Node, INode};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NodeHello {
    base: Base<Node>
}

#[godot_api]
impl NodeHello {
    #[func]
    fn say_hello(&self) {
        godot_print!("NodeHello...say_hello");
    }
}

#[godot_api]
impl INode for NodeHello {
    fn init(base: Base<Node>) -> Self {
        godot_print!("NodeHello...init");
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("NodeHello...ready");
        self.say_hello();
    }
}
"#; 
        // 封装逻辑以使用 ? 语法
        let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            fs::File::create(&file_node_hello_path)?; // 注意加了 &
            std::fs::write(&file_node_hello_path, content)?; // 现在可以正常使用了
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                godot_print!("create file node hello 创建完毕了");
                self.send_message_to_rich(format!("create file node hello 创建完毕了"));
                self.cargo_build("CARGO_BUILD_PROJECT".to_string());
            }
            Err(_e) => {
                godot_print!("create file node hello  创建失败了");
                self.send_message_to_rich(format!("create file node hello  创建失败了"));
            }
        }
    }




    
    // 子线程中, 启动 godot 工具
    #[func]
    fn start_up_godot(&mut self) {
        godot_print!("start up godot 创建完毕了");
        // 1. 定义目标工作目录
        let path_godot = self.path_godot.clone();
        let godot_progect_path = format!("{}/godot/", self.work_space);

         godot_print!("start up godot path: {}" , path_godot);

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx); // 将接收端交给主线程轮询
        std::thread::spawn(move || {
            // 1. 发送开始信号
            let _ = tx.send("任务开始...".to_string());
 
            // 2. 创建并配置命令
            let child = Command::new(path_godot) // 确保 godot 已加入环境变量，否则请使用绝对路径
                .arg("godot")
                .arg("--editor")                    // 传递编辑器参数
                .current_dir(godot_progect_path)    // 设置执行路径（相当于在该文件夹下打开终端）
                .spawn();                          // 执行并等待返回结果

            match child {
                Ok(_child_process) => {
                    // 3. 进程启动成功，立即发送信号
                    let _ = tx.send("GODOT_START_UP".to_string());
                }
                Err(e) => {
                    let _ = tx.send(format!("PROCESS_FAIL: {}", e));
                }
            }
        });
    }
}