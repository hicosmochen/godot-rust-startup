use godot::prelude::*;
use godot::classes::{Button, IButton, PackedScene, LineEdit}; // 导入需要的 UI 类
use godot::classes::os::SystemDir;
use godot::classes::ConfigFile;  // 正确导入 配置文件信息
use godot::global::Error;        // 正确导入 Godot 的全局错误枚举

use std::sync::mpsc;
use std::os::windows::process::CommandExt; // 必须导入

// 记得导入你的自定义类
use crate::menu::my_file_dialog::MyFileDialog;
use crate::project::project_richtext::ProjectRichTextLabel;
use crate::secure::secure_storage::SecureStorage;


// 定义 Button 的枚举类型, 让godot可以进行外部赋值（赋值的内容是指定的数据值）
#[derive(GodotConvert, Var, Export, Default, Copy, Clone, Debug)]
// 告诉 Godot 以整数的形式存储索引
#[godot(via = i64)]
pub enum ButtonKind {
    #[default]
    SelectWorkSpace,
    RustRootPath,
    GdextFileName,
    NeedCreateDemo,
    StartCreatProject,
    Cancel,
    CancelCreatDemo,
    ConfirmPathRust,
    ConfirmNameGdext,
    ConfirmCreatDemo,
}


#[derive(GodotClass)]
#[class(base=Button)] 
pub struct ProjectButton {

    pub base: Base<Button>,

    #[export]
    pub kind: ButtonKind,

    // 外部加入显示区域的 富文本信息
    #[export]
    pub label_rich: Option<Gd<ProjectRichTextLabel>>,

    // 使用 Option 包装，因为在 init 时还没有通道
    receiver: Option<mpsc::Receiver<String>>,
}


#[godot_api]
impl IButton for ProjectButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            kind: ButtonKind::SelectWorkSpace,   // 默认初始化
            label_rich: None, // 初始化必须为 None，等待 Godot 注入,
            receiver : None,
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
impl ProjectButton {


    #[func]
    fn on_button_pressed(&mut self){
         // 根据 kind 执行逻辑
        match self.kind {
            ButtonKind::SelectWorkSpace => {
                // 这里需要打开文件目录选择弹窗
                // 启动文件类型的对话框
                self.open_dir_dialog(r"选择工作空间".to_string());
            },
            ButtonKind::RustRootPath => {
                let scene_path = String::from("res://scene/dialog_name_rust.tscn");
                self.create_dialog_by_scene(scene_path);
            },
            ButtonKind::GdextFileName => { 
                let scene_path = String::from("res://scene/dialog_name_gdext.tscn");
                self.create_dialog_by_scene(scene_path);
            },
            ButtonKind::NeedCreateDemo => { 
                let scene_path = String::from("res://scene/dialog_create_demo.tscn");
                self.create_dialog_by_scene(scene_path);
            },
            ButtonKind::StartCreatProject => self.start_create_project(),
            ButtonKind::Cancel => {
                // 关闭当前的场景
                self.close_project_dialog();
            },
            ButtonKind::ConfirmPathRust => {
                // 1. 获取父节点
                if let Some(parent) = self.base().get_parent() {
                    // 2. 尝试获取同级的 LineEdit
                    // 注意："LineEdit" 必须与场景面板中的名称一致
                    if let Some(line_edit) = parent.try_get_node_as::<LineEdit>("LineEdit") {
                        // 3. 访问信息
                        let text = line_edit.get_text();
                        // 判断内容如果为空的情况下, 给出提示信息
                        if text.is_empty() {
                            self.send_message_to_rich(format!("RUST的根目录不正确, 内容不能为空"));
                        } else {
                            //------------------------------------------------------------
                            // 构建配置对象
                            let mut config = ConfigFile::new_gd();
                            // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
                            let _ = config.load("res://config.cfg"); 
                            config.set_value("Editor", "rust_root", &text.to_variant());
                            // 保存并检查结果
                            let result = config.save("res://config.cfg");
                            if result == Error::OK {
                                godot_print!("路径已成功保存: {}", text);
                                self.send_message_to_rich(format!("RUST根目录名称: {text}"));
                            } else {
                                self.send_message_to_rich(format!("存储失败"));
                            }
                            //------------------------------------------------------------
                        }
                    }else {
                        godot_warn!("找不到名为 LineEdit 的同级节点");
                    };   
                }
                // 关闭当前的场景
                self.close_project_dialog();
            }
            ButtonKind::ConfirmNameGdext => {
                // 1. 获取父节点
                if let Some(parent) = self.base().get_parent() {
                    // 2. 尝试获取同级的 LineEdit
                    // 注意："LineEdit" 必须与场景面板中的名称一致
                    if let Some(line_edit) = parent.try_get_node_as::<LineEdit>("LineEdit") {
                        // 3. 访问信息
                        let text = line_edit.get_text();
                        // 判断内容如果为空的情况下, 给出提示信息
                        if text.is_empty() {
                            self.send_message_to_rich(format!("GDEXT的名称不正确, 内容不能为空"));
                        } else {
                            //------------------------------------------------------------
                            // 构建配置对象
                            let mut config = ConfigFile::new_gd();
                            // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
                            let _ = config.load("res://config.cfg"); 
                            config.set_value("Editor", "gdext_name", &text.to_variant());
                            // 保存并检查结果
                            let result = config.save("res://config.cfg");
                            if result == Error::OK {
                                godot_print!("路径已成功保存: {}", text);
                                self.send_message_to_rich(format!("GDEXT的名称: {text}"));
                            } else {
                                self.send_message_to_rich(format!("存储失败"));
                            }
                            //------------------------------------------------------------
                        }
                    }else {
                        godot_warn!("找不到名为 LineEdit 的同级节点");
                    };   
                }
                // 关闭当前的场景
                self.close_project_dialog();
            }
            ButtonKind::ConfirmCreatDemo => {
                //------------------------------------------------------------
                // 构建配置对象
                let mut config = ConfigFile::new_gd();
                // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
                let _ = config.load("res://config.cfg"); 
                config.set_value("Editor", "create_demo", &true.to_variant());
                // 保存并检查结果
                let result = config.save("res://config.cfg");
                if result == Error::OK {
                    self.send_message_to_rich(format!("创建案例项目: 是"));
                } else {
                    self.send_message_to_rich(format!("存储失败"));
                }
                //------------------------------------------------------------
                // 关闭当前的场景
                self.close_project_dialog();
            }
            ButtonKind::CancelCreatDemo => {
                //------------------------------------------------------------
                // 构建配置对象
                let mut config = ConfigFile::new_gd();
                // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
                let _ = config.load("res://config.cfg"); 
                config.set_value("Editor", "create_demo", &false.to_variant());
                // 保存并检查结果
                let result = config.save("res://config.cfg");
                if result == Error::OK {
                    self.send_message_to_rich(format!("创建案例项目: 否"));
                } else {
                    self.send_message_to_rich(format!("存储失败"));
                }
                //------------------------------------------------------------
                // 关闭当前的场景
                self.close_project_dialog();
            }
        }
    }

    // 打开文件夹的对话框
    #[func]
    fn open_dir_dialog(&mut self, &title: String){
        // 1. 动态实例化 使用新的 API 名称：from_init_fn
        let mut dialog = Gd::<MyFileDialog>::from_init_fn( |base|{
            MyFileDialog { base, kind: String::from("work_space") }
        });

        // 2. 设置 Godot 属性（可选）  current_dir
        dialog.set_access(godot::classes::file_dialog::Access::FILESYSTEM);
        dialog.set_file_mode(godot::classes::file_dialog::FileMode::OPEN_DIR);
        dialog.set_title(&title);
        dialog.set_use_native_dialog(true);

        let docs_path = godot::classes::Os::singleton().get_system_dir(SystemDir::DOCUMENTS);
        dialog.set_current_dir(&docs_path);
        // 3. 必须先加入场景树
        let dialog_node = dialog.clone().upcast::<Node>();
        self.base_mut().add_child(&dialog_node);
        // 4. 弹出
        dialog.bind_mut().open_dialog();
    }

    #[func]
    fn close_project_dialog(&mut self){
        // 关闭当前的场景
        let mut tree = self.base().get_tree().unwrap();
        // 找到组内所有成员并销毁
        tree.call_group("node_dialog", "queue_free", &[]);
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


    // 通过场景路径创建对话框
    #[func]
    pub fn create_dialog_by_scene(&mut self, scene_path: String){
        let scene = load::<PackedScene>(&scene_path);
        // 1. 实例化为 Node
        let my_node = scene.instantiate_as::<Node>();

        // 2. 获取场景树根节点 (Main Loop 的根 Window)
        if let Some(mut root) = self.base().get_tree().and_then(|t| t.get_root()) {
            // 3. 将 Node 挂载到根部
            root.add_child(&my_node);
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
    pub fn start_create_project(&mut self){
        godot_print!("按钮被点击了..start_create_project.StartCreatProject");
        // 获取到前面存储的所有数据
        // 1、Godot 启动文件
        let path_godot = SecureStorage::get("path_godot");
        let path_rust = SecureStorage::get("path_rust");
        // 构建配置对象
        let mut config = ConfigFile::new_gd();
        // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
        let _ = config.load("res://config.cfg"); 
        // 带默认值的安全获取（推荐方式）
        let work_space = config
            .get_value_ex("Editor", "work_space")
            .default(&"".to_variant())  
            .done()
            .to::<String>();

        let rust_root = config
            .get_value_ex("Editor", "rust_root")
            .default(&"".to_variant())  
            .done()
            .to::<String>();

        let gdext_name = config
            .get_value_ex("Editor", "gdext_name")
            .default(&"".to_variant())  
            .done()
            .to::<String>();

        let _create_demo = config
            .get_value_ex("Editor", "create_demo")
            .default(&false.to_variant())  
            .done()
            .to::<bool>();

        if path_godot.is_empty(){
            self.send_message_to_rich(format!("Godot 的路径不能为空"));
            return
        }
        
        if path_rust.is_empty(){
            self.send_message_to_rich(format!("Rust 的路径不能为空"));
            return
        }
        
        if work_space.is_empty(){
            self.send_message_to_rich(format!("工作空间 的路径不能为空"));
            return
        }
        
        if gdext_name.is_empty(){
            self.send_message_to_rich(format!("gdext的名称不能为空"));
            return
        }

        // ------------------------------------------------------------
         godot_print!("开始创建项目");
        // 创建项目
        self.creat_rust_project(path_rust, work_space, rust_root);
    }


    // 创建 rust 项目
    #[func]
    fn creat_rust_project(&mut self, path_rust : String, work_space: String,  rust_root : String) {
        // cargo new  myrust   --lib
        // 需要执行上面的命令  C:/Users/Administrator/.cargo/bin/cargo.exe
        let cargo_path =  path_rust + "/bin/cargo.exe";
        self.send_message_to_rich(format!("cargo: {cargo_path}"));

        // 克隆变量以进入线程闭包
        let work_space_clone = work_space.clone();

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
        let path_rust = SecureStorage::get("path_rust");
        let cargo_path =  path_rust + "/bin/cargo.exe";
        // 构建配置对象
        let mut config = ConfigFile::new_gd();
        // 尝试加载旧配置（忽略“文件不存在”的错误，因为第一次运行肯定没有）
        let _ = config.load("res://config.cfg"); 
        // 带默认值的安全获取（推荐方式）
        let work_space = config
            .get_value_ex("Editor", "work_space")
            .default(&"".to_variant())  
            .done()
            .to::<String>();

        let rust_root = config
            .get_value_ex("Editor", "rust_root")
            .default(&"".to_variant())  
            .done()
            .to::<String>();

        // 需要执行下面的命令
        // 克隆变量以进入线程闭包
        let work_space_clone = work_space  + "/"+ &rust_root;

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

    #[func]
    fn modify_cargo_toml(&mut self) {
        let mut config = ConfigFile::new_gd();
        let _ = config.load("res://config.cfg");

        // 错误 1 修复：.default() 需要传入引用 &Variant，所以加上 &
        let work_space = config
            .get_value_ex("Editor", "work_space")
            .default(&"".to_variant()) 
            .done()
            .to::<String>();

        let rust_root = config
            .get_value_ex("Editor", "rust_root")
            .default(&"".to_variant()) 
            .done()
            .to::<String>();

        let cargo_toml_path = format!("{}/{}/Cargo.toml", work_space, rust_root);

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
                godot_print!("结果成功");
                self.send_message_to_rich(format!("modify cargo toml 创建完毕了"));
            }
            Err(_e) => {
                godot_print!("结果失败");
                self.send_message_to_rich(format!("modify cargo toml 创建失败了"));
            }
        }
    }
}


