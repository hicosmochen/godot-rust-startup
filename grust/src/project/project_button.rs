use godot::prelude::*;
use godot::classes::{Button, IButton, Theme}; // 导入需要的 UI 类
use godot::classes::os::SystemDir;
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
        godot_print!("按钮"); 
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
                godot_print!("按钮被点击了...SelectWorkSpace");
                // 这里需要打开文件目录选择弹窗
                // 启动文件类型的对话框
                self.open_dir_dialog(r"选择工作空间".to_string())
            },
            ButtonKind::RustRootPath => godot_print!("按钮被点击了...RustRootPath"),
            ButtonKind::GdextFileName => godot_print!("按钮被点击了...GdextFileName"),
            ButtonKind::NeedCreateDemo => godot_print!("按钮被点击了...NeedCreateDemo"),
            ButtonKind::StartCreatProject => godot_print!("按钮被点击了...StartCreatProject"),
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
}


