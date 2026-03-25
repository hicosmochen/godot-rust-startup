use godot::prelude::*;
use godot::classes::{Button, IButton, PackedScene, LineEdit}; // 导入需要的 UI 类
use godot::classes::os::SystemDir;
use godot::classes::ConfigFile;  // 正确导入 配置文件信息
use godot::global::Error;        // 正确导入 Godot 的全局错误枚举

// 记得导入你的自定义类
use crate::menu::my_file_dialog::MyFileDialog;
use crate::project::project_richtext::ProjectRichTextLabel;


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
}


#[godot_api]
impl IButton for ProjectButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            kind: ButtonKind::SelectWorkSpace,   // 默认初始化
            label_rich: None, // 初始化必须为 None，等待 Godot 注入,
        }
    }

    fn ready(&mut self) {
        // 绑定信号
        let button_pressed =  self.base().callable("on_button_pressed");
        self.base_mut().connect("pressed", &button_pressed);
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
                self.open_dir_dialog(r"选择工作空间".to_string())
            },
            ButtonKind::RustRootPath => {
                let scene_path = "res://scene/dialog_name_rust.tscn";
                let scene = load::<PackedScene>(scene_path);
                // 1. 实例化为 Node
                let my_node = scene.instantiate_as::<Node>();

                // 2. 获取场景树根节点 (Main Loop 的根 Window)
                if let Some(mut root) = self.base().get_tree().and_then(|t| t.get_root()) {
                    // 3. 将 Node 挂载到根部
                    root.add_child(&my_node);
                }
            },
            ButtonKind::GdextFileName => {
                let scene_path = "res://scene/dialog_name_gdext.tscn";
                let scene = load::<PackedScene>(scene_path);
                // 1. 实例化为 Node
                let my_node = scene.instantiate_as::<Node>();

                // 2. 获取场景树根节点 (Main Loop 的根 Window)
                if let Some(mut root) = self.base().get_tree().and_then(|t| t.get_root()) {
                    // 3. 将 Node 挂载到根部
                    root.add_child(&my_node);
                }
            },
            ButtonKind::NeedCreateDemo => godot_print!("按钮被点击了...NeedCreateDemo"),
            ButtonKind::StartCreatProject => godot_print!("按钮被点击了...StartCreatProject"),
            ButtonKind::Cancel => {
                // 关闭当前的场景
                self.close_project_dialog();
            },
            ButtonKind::ConfirmPathRust => {
                godot_print!("按钮被点击了...确认按钮");
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
                godot_print!("按钮被点击了...确认按钮");
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
                godot_print!("按钮被点击了...确认创建案例");
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
}


