use godot::prelude::*;
use godot::classes::{Button, IButton}; // 导入需要的 UI 类

use godot::classes::ConfigFile;  // 正确导入 配置文件信息

use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::mpsc;
use std::os::windows::process::CommandExt; // 必须导入
use std::io::Write;
use std::env;
use std::path::PathBuf;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;


use crate::project::project_richtext::ProjectRichTextLabel;
use crate::project::project_progress::ProjectProgressBar;
use crate::secure::secure_storage::SecureStorage;



#[derive(GodotClass)]
#[class(base=Button)] 
pub struct ProjectButtonCreate {
    base: Base<Button>,
    // 外部加入显示区域的 富文本信息
    #[export]
    pub label_rich: Option<Gd<ProjectRichTextLabel>>,

    // 使用 Option 包装，因为在 init 时还没有通道
    receiver: Option<mpsc::Receiver<BuildEvent>>,

    path_rust  : String,
    path_godot : String,
    work_space : String,
    rust_root  : String,
    gdext_name : String,
    create_demo : bool,
    time_accumulator: f64, // 新增：用于计时
}




// 定义消息类型，每一项都是一个“键”，括号内是它携带的“值”
#[derive(Debug)]
enum BuildEvent {
    Status(String),           // 键：状态，值：字符串描述
    Progress(f64),            // 键：进度，值：浮点数
    Error(String, String),    // 键：错误，值：(错误类型, 详细描述)
    Log(String),              // 键：日志，值：单行文本
    Complete(String),         // 键：完成，值：动作名称
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
            time_accumulator: 0.5,
        }
    }

    fn ready(&mut self) {
        // 绑定信号
        let button_pressed =  self.base().callable("on_button_pressed");
        self.base_mut().connect("pressed", &button_pressed);
    }


    fn process(&mut self, _delta: f64) {
        let mut events = Vec::new();
        // 1. 快速检查通道，仅不可变借用 self.receiver
        if let Some(ref rx) = self.receiver {
            while let Ok(event) = rx.try_recv() {
                events.push(event);
            }
        }
        // 2. 处理键值对
        for event in events {
            match event {
                // 匹配键: Status 提取值: msg
                BuildEvent :: Status(status) => {
                    godot_print!("状态更新: {}", status); 
                    match status.as_str() {
                        "STATUS_CREATE_RUST" => {
                            let translated_text_create_rust_project = self.base_mut().tr("create_rust_project");
                            self.send_message_to_rich(format!("{translated_text_create_rust_project}"), 0);
                        },
                        "STATUS_COMPILING_RUST" => {
                            let translated_text_compiling_rust = self.base_mut().tr("compiling_rust");
                            self.send_message_to_rich(format!("{translated_text_compiling_rust}"), 0);
                        },
                        "STATUS_START_GODOT" => {
                            let translated_text_starting_godot = self.base_mut().tr("starting_godot");
                            self.send_message_to_rich(format!("{translated_text_starting_godot}"), 0);
                        },
                        _=> godot_print!("未匹配到合适的类型"),
                    }
                },
                // 匹配键: Progress 提取值 Val
                BuildEvent :: Progress(val) => {
                    self.update_progress_value(val);
                },
                // 匹配键：Complete，根据不同的动作值跳转逻辑
                BuildEvent::Complete(action) => {
                    match action.as_str() {
                        "CARGO_SUCCESS" => {
                            self.cargo_add_godot();
                        },
                        "ADD_GODOT_SUCCESS" => {
                            self.modify_cargo_toml();
                        },
                         "CARGO_BUILD_INIT" => {
                            self.create_godot_project();
                        },
                        "CARGO_BUILD_PROJECT" => {
                            self.create_main_scene();
                        },
                        "GODOT_START_UP" => {
                            self.base_mut().set_disabled(false);
                            self.send_message_to_rich(format!("godot project start up done"), 0);
                            self.send_message_to_rich(format!("all operations have been completed."), 2);
                            self.send_message_to_rich(format!("------------------------------------------------------"), 0);
                        },
                        _ => godot_print!("未知动作完成: {}", action),
                    }
                }
                // 匹配键：Error，提取值：err_type 和 detail
                BuildEvent::Error(err_type, detail) => {
                    match err_type.as_str() {
                        "WORKSPACE_INVALID_PATH" => {
                            let translated_text_work_space_does_not_exist = self.base_mut().tr("work_space_does_not_exist");
                            self.send_message_to_rich(format!("{}: {}", translated_text_work_space_does_not_exist.to_string(), self.work_space), 1);
                            self.base_mut().set_disabled(false);
                        },
                        _=> {
                            godot_print!("发生错误 [{}]: {}", err_type, detail);
                            self.send_message_to_rich(format!("{}: {}", err_type, detail), 1);
                            self.base_mut().set_disabled(false);
                        },
                    }
                }

                // 匹配键：Log，提取值：text
                BuildEvent::Log(text) => {
                    self.send_message_to_rich(text, 0);
                }
            }
        }
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectButtonCreate {

    // 发送信息给 富文本显示内容
    #[func]
    pub fn send_message_to_rich(&mut self, message: String, log_type: i64){
        // 1. 将字符串映射为 StringName
        let method_name = match log_type {
            1 => "on_add_log_fail",
            2 => "on_add_log_success",
            3 => "on_add_log_emphasize",
            _ => "on_add_log", // 默认情况
        };

        // 2. 安全获取 SceneTree
        if let Some(mut tree) = self.base().get_tree() {
            // 3. 使用 call_group 发送信号
            // 注意：method_name 需要转换为 StringName
            tree.call_group(
                "log_receivers", 
                method_name, 
                &[message.to_variant()]
            );
        } else {
            godot_warn!("无法获取 SceneTree，消息发送失败");
        }
    }


    #[func]
    fn on_button_pressed(&mut self){
        self.base_mut().set_disabled(true);
        if self.start_create_project(){
            godot_print!("开始创建项目");
            self.creat_rust_project();
        }else{
            self.base_mut().set_disabled(false);
            godot_print!("条件不满足");
        }
    }


    // 获取数据, 对数据进行校验
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
            self.send_message_to_rich(format!("Godot 的路径不能为空"), 0);
            return false;
        }
        
        if self.path_rust.is_empty(){
            self.send_message_to_rich(format!("Rust 的路径不能为空"), 0);
            return false;
        }
        
        if self.work_space.is_empty(){
            self.send_message_to_rich(format!("工作空间 的路径不能为空"), 0);
             return false;
        }
        
        if self.gdext_name.is_empty(){
            self.send_message_to_rich(format!("gdext的名称不能为空"), 0);
             return false;
        }
        // 更新进度值
        self.update_progress_value(5.0);

        let cargo_path =  format!("{}/bin/cargo.exe", self.path_rust);
        let godot_path = format!("{}", self.path_godot);


        let translated_text_cargo_path = self.base_mut().tr("cargo_path");
        self.send_message_to_rich(format!("{translated_text_cargo_path}: {cargo_path}"), 3);
        let translated_text_godot_path = self.base_mut().tr("godot_path");
        self.send_message_to_rich(format!("{translated_text_godot_path}: {godot_path}"), 3);
        self.send_message_to_rich(format!("------------------------------------------------------"),  0);

        return true;
    }


    // 创建 rust 项目
    #[func]
    fn creat_rust_project(&mut self) {
        let cargo_path =  format!("{}/bin/cargo.exe", self.path_rust);

        // 克隆变量以进入线程闭包
        let rust_root = self.rust_root.clone();
        let work_space_clone = self.work_space.clone();

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx); // 将接收端交给主线程轮询
        std::thread::spawn(move || {
            // 1. 发送开始信号
            let _ = tx.send(BuildEvent::Status(format!("STATUS_CREATE_RUST")));

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
                Ok(_) => { 
                    // 完成
                    let _ = tx.send(BuildEvent::Complete(format!("CARGO_SUCCESS")));
                }
                Err(e) => { 
                    let _ = tx.send(BuildEvent::Error(format!("CARGO_FAIL"), format!("{}", e))); 
                    if let Some(267) = e.raw_os_error() {
                        // 专门处理“目录名称无效”的情况
                        let _ = tx.send(BuildEvent::Error(format!("WORKSPACE_INVALID_PATH"), "".to_string()));
                    }
                }
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
            let _ = tx.send(BuildEvent::Status(format!("正在关联.godot.到.rust...")));

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
                        let _ = tx.send(BuildEvent::Complete(format!("ADD_GODOT_SUCCESS")));
                    } else {
                        // 关键修正 2: 捕获并发送真正的错误信息
                        let error_msg = String::from_utf8_lossy(&out.stderr);
                        let _ = tx.send(BuildEvent::Error(format!("CARGO_FAIL"), format!("{}", error_msg))); 
                    }
                }
                Err(e) => {
                    let _ = tx.send(BuildEvent::Error(format!("PROCESS_FAIL"), format!("{}", e))); 
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
                godot_print!("modify cargo toml create done");
                self.send_message_to_rich(format!("modify cargo toml create done"), 0);
                // 更新进度值
                self.update_progress_value(25.0);
                self.modify_lib_rs();
            }
            Err(_e) => {
                godot_print!("modify cargo toml create fail");
                self.send_message_to_rich(format!("modify cargo toml create fail"), 1);
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
                // 更新进度值
                self.update_progress_value(30.0);
                godot_print!("modify lib rs done");
                self.send_message_to_rich(format!("modify lib rs done"), 0);
                self.cargo_build("CARGO_BUILD_INIT".to_string());
            }
            Err(_e) => {
                godot_print!("modify lib rs fail");
                self.send_message_to_rich(format!("modify lib rs fail"), 1);
            }
        }
    }




    
    // 自动化实现 （执行命令） cargo  build
    #[func]
    fn cargo_build(&mut self, action: String) {
        let cargo_path = format!("{}/bin/cargo.exe", self.path_rust);
        let work_space_clone = format!("{}/{}", self.work_space, self.rust_root);

        let (tx, rx) = std::sync::mpsc::channel();
        self.receiver = Some(rx);

        thread::spawn(move || {
            let _ = tx.send(BuildEvent::Status(format!("STATUS_COMPILING_RUST")));
            
            #[cfg(windows)]
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // 1. 配置命令，将输出重定向到管道
            let mut command = Command::new(cargo_path);
            command.arg("build")
                .current_dir(work_space_clone)
                .stdout(Stdio::piped()) // 捕获标准输出
                .stderr(Stdio::piped()); // 捕获错误输出（Cargo 编译进度通常在 stderr）

            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                command.creation_flags(CREATE_NO_WINDOW);
            }

            // 2. 启动进程
            let mut child = match command.spawn() {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(BuildEvent::Error(format!("无法启动进程"), format!("{}", e))); 
                    return;
                }
            };

            // 3. 实时读取 stderr (Cargo 的主要输出在这里)
            let stderr = child.stderr.take().unwrap();
            let tx_stderr = tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let _ = tx_stderr.send(BuildEvent::Log(format!("{}", l))); // 实时发送每一行
                    }
                }
            });

            // 4. 实时读取 stdout
            let stdout = child.stdout.take().unwrap();
            let tx_stdout = tx.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        let _ = tx_stdout.send(BuildEvent::Log(format!("{}", l))); // 实时发送每一行
                    }
                }
            });

            // 5. 等待进程结束
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        let _ = tx.send(BuildEvent::Complete(format!("{}", action)));
                    } else {
                        let _ = tx.send(BuildEvent::Error(format!("编译失败，请检查上方日志"), format!("{}", ""))); 
                    }
                }
                Err(e) => {
                    let _ = tx.send(BuildEvent::Error(format!("等待进程时出错"), format!("{}", e))); 
                }
            }
        });
    }




    // 创建 godot 项目
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
            self.send_message_to_rich(format!("dir godot create success"), 0);
            godot_print!("dir godot create success");
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
                // 更新进度值
                self.update_progress_value(50.0);
                godot_print!("project.godot create success");
                self.send_message_to_rich(format!("project.godot create success"), 0);
                self.create_file_gdextension();
            }
            Err(e) => {
                godot_print!("project.godot create fail: {}", e);
                self.send_message_to_rich(format!("project.godot create fail: {}", e), 0);
            }
        }
    }


    // 创建 gdextension 文件, 这里将 gdextension 存放到 addons 文件夹下面
    #[func]
    fn create_file_gdextension(&mut self) {
        godot_print!("需要创建文件 create file gdextension 文件中写入数据 ");
        // 1. 先构建 addons 文件夹的路径
        let addons_dir = format!("{}/godot/addons", self.work_space);
        // 2. 递归创建文件夹
        // create_dir_all 会处理多级不存在的目录
        if let Err(e) = fs::create_dir_all(&addons_dir) {
            eprintln!("无法创建文件夹: {}", e);
            // 根据你的逻辑处理错误，例如 return Err(e)
        }

        // 定义需要操作的路径
        let file_gdextension_path = format!("{}/{}.gdextension", addons_dir, self.gdext_name);
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
                // 更新进度值
                self.update_progress_value(55.0);
                godot_print!("create file gdextension create success");
                self.send_message_to_rich(format!("create file gdextension create success"), 0);
                // 准备修改配置文件
                self.modify_godot_projects();
            }
            Err(_e) => {
                godot_print!("create file gdextension create fail");
                self.send_message_to_rich(format!("create file gdextension create fail"), 1);
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
                // 更新进度值
                self.update_progress_value(60.0);
                godot_print!("modify godot projects done");
                self.send_message_to_rich(format!("modify godot projects done"), 0);
                self.send_message_to_rich(format!("is need create demo : {}", self.create_demo), 0);

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
                godot_print!("modify godot projects fail");
                self.send_message_to_rich(format!("modify godot projects fail"), 1);
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
                // 更新进度值
                self.update_progress_value(65.0);
                godot_print!("lib.rs append module success");
                self.send_message_to_rich(format!("lib.rs append module success"), 0);
                self.create_file_node_hello();
            },
            Err(e) => {
                godot_print!("lib.rs append module fail: {}", e);
                self.send_message_to_rich(format!("lib.rs append module fail"), 1);
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
                // 更新进度值
                self.update_progress_value(70.0);
                godot_print!("create file node hello done");
                self.send_message_to_rich(format!("create file node hello done"), 0);
                self.cargo_build("CARGO_BUILD_PROJECT".to_string());
            }
            Err(_e) => {
                godot_print!("create file node hello fail");
                self.send_message_to_rich(format!("create file node hello fail"), 1);
            }
        }
    }


    // 创建主场景
    #[func]
    fn create_main_scene(&mut self){
        // 1. 先构建 scenes 文件夹的路径
        let scenes_dir = format!("{}/godot/scenes", self.work_space);
        // 2. 递归创建文件夹
        // create_dir_all 会处理多级不存在的目录
        if let Err(e) = fs::create_dir_all(&scenes_dir) {
            eprintln!("无法创建文件夹: {}", e);
        }

        // 1. 定义场景路径（通常在项目根目录下的 scenes 或直接在 res://）
        // 注意：这里的路径应指向你的 Godot 项目根目录
        let scene_path = format!("{}/main.tscn", scenes_dir);

        // 2. 构造 TSCN 内容
        // [node name="NodeHello" type="NodeHello"] 这里的 type 必须与你 Rust 中 #[class(base=Node)] 定义的类名一致
        let scene_content = r#"[gd_scene format=3 uid="uid://c8x7y6z5w4v3u"]

    [node name="MainRoot" type="NodeHello"]
    "#;

        // 3. 写入文件（覆盖模式）
         let execute_modify = || -> Result<(), Box<dyn std::error::Error>> {
            fs::File::create(&scene_path)?;
            std::fs::write(&scene_path, scene_content)?; // 现在可以正常使用了
            println!("场景文件 main.tscn 已创建");
            Ok(())
        };

        match execute_modify() {
            Ok(_) => {
                // 更新进度值
                self.update_progress_value(80.0);
                godot_print!("create main scene done");
                self.send_message_to_rich(format!("create main scene done"), 0);
                self.set_as_main_scene();
            }
            Err(_e) => {
                godot_print!("create main scene 创建失败了");
                self.send_message_to_rich(format!("create main scene fail"), 1);
            }
        }
    }



    // 设置场景为主场景
    #[func]
    fn set_as_main_scene(&mut self) {
        let config_path = format!("{}/godot/project.godot", self.work_space);
        let main_scene_line = r#"run/main_scene="res://scenes/main.tscn""#;

        // 1. 尝试读取文件逻辑 (使用 match 代替 ?，因为当前函数不返回 Result)
        let modify_result = (|| -> Result<(), Box<dyn std::error::Error>> {
            let mut content = std::fs::read_to_string(&config_path)?;

            // 2. 修改逻辑
            if !content.contains("run/main_scene") {
                if let Some(pos) = content.find("[application]") {
                    let insert_pos = content[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(pos + "[application]".len());
                    content.insert_str(insert_pos, &format!("{}\n", main_scene_line));
                } else {
                    content.push_str(&format!("\n[application]\n{}\n", main_scene_line));
                }
            } else {
                // 如果你希望路径不对时也能自动修复，可以取消下面这行的注释：
                // content = content.lines().map(|line| if line.starts_with("run/main_scene") { main_scene_line } else { line }).collect::<Vec<_>>().join("\n");
            }

            // 3. 写入文件 (fs::write 会自动覆盖，不需要先 File::create)
            std::fs::write(&config_path, content)?;
            Ok(())
        })(); // 注意这里的 () 表示立即执行这个闭包

        // 4. 在函数内部根据结果进行 UI 反馈
        match modify_result {
            Ok(_) => {
                // 更新进度值
                self.update_progress_value(90.0);
                godot_print!("set as main scene success");
                self.send_message_to_rich(format!("set as main scene success"), 0);
                self.start_up_godot();
            }
            Err(e) => {
                // 打印具体错误 e 方便你调试（比如文件没找到或拒绝访问）
                godot_print!("set as main scene 失败: {:?}", e);
                self.send_message_to_rich(format!("set as main scene fail: {}", e), 1);
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

         godot_print!("start up godot path: {}" , godot_progect_path);

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx); // 将接收端交给主线程轮询
        std::thread::spawn(move || {
            // 1. 发送开始信号
            let _ = tx.send(BuildEvent::Status(format!("STATUS_START_GODOT")));
            
            // 2. 创建并配置命令
            let child = Command::new(path_godot) // 确保 godot 已加入环境变量，否则请使用绝对路径
                .arg("godot")
                .arg("--editor")                    // 传递编辑器参数
                .current_dir(godot_progect_path)    // 设置执行路径（相当于在该文件夹下打开终端）
                .spawn();                          // 执行并等待返回结果

            match child {
                Ok(_child_process) => {
                    // 3. 进程启动成功，立即发送信号
                    let _ = tx.send(BuildEvent::Complete(format!("GODOT_START_UP")));
                    let _ = tx.send(BuildEvent::Progress(100.0));
                }
                Err(e) => {
                    let _ = tx.send(BuildEvent::Error(format!("PROCESS_FAIL"), format!("{}", e))); 
                }
            }
        });
    }

    // 更新进度值的操作
    #[func]
    fn update_progress_value(&mut self, delta: f64) {
        // 累加每次执行的时间间隔
        self.time_accumulator += delta;

        // 只有当时间超过 0.5 秒时才执行更新
        if self.time_accumulator >= 0.5 {
            let mut progress_bar = self.base().get_node_as::<ProjectProgressBar>("../ProjectProgressBar");
            progress_bar.bind_mut().set_progress(delta);
            
            // 重置计时器（或者减去 0.5 以保持更高精度）
            self.time_accumulator = 0.0;
        }
    }
}