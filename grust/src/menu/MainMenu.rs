use godot::prelude::*;
use godot::classes::{ColorRect, IColorRect, MenuButton, Theme, DisplayServer}; // 导入需要的 UI 类
use godot::classes::Os;
use godot::classes::os::SystemDir;
// 记得导入你的自定义类
use crate::menu::MyFileDialog::MyFileDialog;



#[derive(GodotClass)]
#[class(base=ColorRect)] 
pub struct MainMenu {
    base: Base<ColorRect>
}


#[godot_api]
impl IColorRect for MainMenu {
    fn init(base: Base<ColorRect>) -> Self {
        godot_print!("rust ColorRect 脚本"); 
        Self {base}
    }


    fn ready(&mut self) {
        // 1. 实例化 MenuButton
        let mut menu_button_config = MenuButton::new_alloc();
        menu_button_config.set_text("环境配置 (config)");

        let mut menu_button_project = MenuButton::new_alloc();
        menu_button_project.set_text("项目创建 (project)");

        let mut menu_button_about = MenuButton::new_alloc();
        menu_button_about.set_text("关于软件 (about)");

        let mut menu_button_exit = MenuButton::new_alloc();
        menu_button_exit.set_text("退出软件 (exit)");
        
        // 设置位置
        menu_button_config.set_position(Vector2::new(20.0, 20.0));
        menu_button_project.set_position(Vector2::new(350.0, 20.0));
        menu_button_about.set_position(Vector2::new(700.0, 20.0));
        menu_button_exit.set_position(Vector2::new(1050.0, 20.0));

        // 2. 获取内置的 PopupMenu 引用
        // 注意：MenuButton 内部已经自动创建了一个 PopupMenu
        let mut menu_popup_config = menu_button_config.get_popup().unwrap();
        let mut menu_popup_about = menu_button_about.get_popup().unwrap();
        let mut menu_popup_exit = menu_button_exit.get_popup().unwrap();

        // 3. 添加菜单项
        menu_popup_config.add_item("Godot启动文件");
        menu_popup_config.set_item_id(0, 1001);
        menu_popup_config.add_separator();           // 分割线
        menu_popup_config.add_item("rust安装路径");
        menu_popup_config.set_item_id(2, 1002);

        menu_popup_about.add_item("版本信息");
        menu_popup_about.set_item_id(0, 3001);
        menu_popup_about.add_separator();           // 分割线
        menu_popup_about.add_item("帮助文档");
        menu_popup_about.set_item_id(2, 3002);
        menu_popup_about.add_separator();           // 分割线
        menu_popup_about.add_item("联系作者");
        menu_popup_about.set_item_id(4, 3003);

        menu_popup_exit.add_item("最小化窗口");
        menu_popup_exit.set_item_id(0, 4001);
        menu_popup_exit.add_separator();           // 分割线
        menu_popup_exit.add_item("退出程序");
        menu_popup_exit.set_item_id(2, 4002);


        // 4. 连接信号
        // 注意：点击事件是发自 PopupMenu 的 "id_pressed" 信号
        menu_popup_config.connect(
            "id_pressed", 
            &(self.base().callable("on_menu_item_pressed"))
        );

        menu_popup_about.connect(
            "id_pressed", 
            &(self.base().callable("on_menu_item_pressed"))
        );

        menu_popup_exit.connect(
            "id_pressed", 
            &(self.base().callable("on_menu_item_pressed"))
        );


        menu_button_project.connect(
            "pressed", 
            &(self.base().callable("on_menu_project_pressed"))
        );


        // 5、将 menu_button_config 添加到自身中
        self.base_mut().add_child(&menu_button_config);
        self.base_mut().add_child(&menu_button_project);
        self.base_mut().add_child(&menu_button_about);
        self.base_mut().add_child(&menu_button_exit);

        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_menu_theme.tres");

        // 应用到主节点，子节点会继承该主题
        self.base_mut().set_theme(&my_theme);
    }
}


// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl MainMenu {
    #[func]
    fn on_menu_item_pressed(&mut self, id: i64) {
        match id {
            1001 => {
                godot_print!("点击了：Godot启动文件");
                // 启动文件类型的对话框
                self.open_file_dialog()
            },
            1002 => godot_print!("点击了：rust安装路径"),
            3001 => godot_print!("点击了：rust版本信息"),
            3002 => godot_print!("点击了：rust帮助文档"),
            3003 => godot_print!("点击了：rust联系作者"),
            4001 => {
                godot_print!("点击了：最小化窗口");
                self.minimize_window()
            },
            4002 => {
                godot_print!("点击了：退出");
                if let Some(mut tree) = self.base_mut().get_tree() {
                            tree.quit();
                }
            },
            _ => godot_print!("未知 ID: {}", id),
        }
    }

    #[func]
    fn on_menu_project_pressed(&mut self){
         godot_print!("点击了：创建项目");
    }

    // 窗口最小化
    #[func]
    fn minimize_window(&mut self) {
        let mut ds = DisplayServer::singleton();
        ds.window_set_mode(godot::classes::display_server::WindowMode::MINIMIZED);
        godot_print!("窗口已最小化");
    }

    // 窗口最大化
    #[func]
    fn maximize_window(&mut self) {
        let mut ds = DisplayServer::singleton();
        ds.window_set_mode(godot::classes::display_server::WindowMode::MAXIMIZED);
        godot_print!("窗口已最大化");
    }


    // 打开文件的对话框
    #[func]
    fn open_file_dialog(&mut self){
        // 1. 动态实例化 使用新的 API 名称：from_init_fn
        let mut dialog = Gd::<MyFileDialog>::from_init_fn( |base|{
            MyFileDialog { base }
        });

        // 2. 设置过滤器
        // 格式说明："*.扩展名 ; 描述" 
        // 注意：分号前后可以有空格，Godot 会自动解析
        let mut filters = PackedStringArray::new();

        let os = Os::singleton();
        let name = os.get_name().to_string(); // 获取系统名称字符串

        match name.as_str() {
            "Windows" => {
                filters.push("*.exe ; Windows 可执行文件"); 
            },
            "macOS" => {
                filters.push("*.app ; macOS 可执行文件"); 
            },
            "Linux" | "FreeBSD" | "NetBSD" | "OpenBSD" | "BSD" => {
                filters.push("*.so ;  Linux 可执行文件"); 
            },
            "Android" => {
                filters.push("*.apk ; Android 可执行文件"); 
                filters.push("*.dex ; Android 可执行文件"); 
            },
            "iOS" => {
                filters.push("*.ipa ; IOS 可执行文件"); 
                filters.push("*.app ; IOS 可执行文件"); 
            },
            _ => godot_print!("未知系统: {}", name),
        }
        dialog.set_filters(&filters);

        // 3. 设置 Godot 属性（可选）  current_dir
        dialog.set_access(godot::classes::file_dialog::Access::FILESYSTEM);
        dialog.set_file_mode(godot::classes::file_dialog::FileMode::OPEN_FILE);
        dialog.set_use_native_dialog(true);

        let docs_path = godot::classes::Os::singleton().get_system_dir(SystemDir::DOCUMENTS);
        dialog.set_current_dir(&docs_path);
        // 4. 必须先加入场景树
        let dialog_node = dialog.clone().upcast::<Node>();
        self.base_mut().add_child(&dialog_node);
        // 5. 弹出
        dialog.bind_mut().open_dialog();
    }
}