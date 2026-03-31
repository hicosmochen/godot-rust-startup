use godot::prelude::*;
use godot::classes::{ColorRect, IColorRect, MenuButton, Theme, DisplayServer}; // 导入需要的 UI 类
use godot::classes::Os;
use godot::classes::os::SystemDir;
// 记得导入你的自定义类
use crate::menu::my_file_dialog::MyFileDialog;
use crate::secure::secure_storage::SecureStorage;

#[derive(GodotClass)]
#[class(base=ColorRect)] 
pub struct MainMenu {
    base: Base<ColorRect>,
    // 保存按钮引用
    btn_config: Option<Gd<MenuButton>>,
    btn_project: Option<Gd<MenuButton>>,
    btn_about: Option<Gd<MenuButton>>,
    btn_exit: Option<Gd<MenuButton>>,
}


#[godot_api]
impl IColorRect for MainMenu {
    fn init(base: Base<ColorRect>) -> Self {
        godot_print!("rust ColorRect 脚本"); 
        Self {
            base,
            btn_config: None,
            btn_project: None,
            btn_about: None,
            btn_exit: None,
        }
    }


    fn ready(&mut self) {
        // 将当前节点, 添加到组当中
        self.base_mut().add_to_group("listener_change_language");
        // 初次创建
        self.on_ready();
    }
}


// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl MainMenu {

    #[func]
    fn on_ready(&mut self){
        // 1. 实例化 MenuButton
        // 如果按钮还没创建，则创建并添加
        if self.btn_config.is_none() {
            // 创建对象
            let mut menu_button_config = MenuButton::new_alloc();
            // 设置位置
            menu_button_config.set_position(Vector2::new(20.0, 20.0));
            // 连接信号
            self.base_mut().add_child(&menu_button_config);
            let mut menu_popup_config = menu_button_config.get_popup().unwrap();
            menu_popup_config.connect(
                "id_pressed", 
                &(self.base().callable("on_menu_item_pressed"))
            );
            // 赋值给配置按钮
            self.btn_config = Some(menu_button_config);
        }

        // 无论是否新创建，每次调用都更新文字（处理翻译）
        if let Some(mut btn) = self.btn_config.clone() {
            let translated_text_config = self.base().tr("environment_configuration");
            btn.set_text(translated_text_config.to_godot());
            // 如果内部的 PopupMenu 条目也需要翻译，在这里 clear() 后重新 add_item
            // 添加子菜单
            let mut menu_popup_config = btn.get_popup().unwrap();
            menu_popup_config.clear();
            let translated_text_rust_path = self.base().tr("rust_installation_path");
            menu_popup_config.add_item(translated_text_rust_path.to_godot());
            menu_popup_config.set_item_id(0, 1001);
            menu_popup_config.add_separator();           // 分割线
            let translated_text_godot_file = self.base().tr("godot_startup_file");
            menu_popup_config.add_item(translated_text_godot_file.to_godot());
            menu_popup_config.set_item_id(2, 1002);
            menu_popup_config.add_separator();           // 分割线
            menu_popup_config.add_item("设置语言");
            menu_popup_config.set_item_id(4, 1003);
        }


        // 如果按钮还没创建，则创建并添加
        if self.btn_project.is_none() {
            let mut menu_button_project = MenuButton::new_alloc();
            menu_button_project.set_position(Vector2::new(350.0, 20.0));
            // 菜单项目被点击 menu_button_about
            menu_button_project.connect(
                "pressed", 
                &(self.base().callable("on_menu_project_pressed"))
            );
            self.base_mut().add_child(&menu_button_project);
            // 赋值给配置按钮
            self.btn_project = Some(menu_button_project);
        }

        // 无论是否新创建，每次调用都更新文字（处理翻译）
        if let Some(mut btn) = self.btn_project.clone() {
            let translated_text_project = self.base().tr("project_creation");
            btn.set_text(translated_text_project.to_godot());
        }


        // 如果按钮还没创建，则创建并添加
        if self.btn_about.is_none() {
            let mut menu_button_about = MenuButton::new_alloc();
            menu_button_about.set_position(Vector2::new(700.0, 20.0));
            let mut menu_popup_about = menu_button_about.get_popup().unwrap();
            // 菜单关于被点击
            menu_button_about.connect(
                "pressed", 
                &(self.base().callable("on_menu_about_pressed"))
            );
            menu_popup_about.connect(
                "id_pressed", 
                &(self.base().callable("on_menu_item_pressed"))
            );
            self.base_mut().add_child(&menu_button_about);
            // 赋值给配置按钮
            self.btn_about = Some(menu_button_about);
        }

        // 无论是否新创建，每次调用都更新文字（处理翻译）
        if let Some(mut btn) = self.btn_about.clone() {
            let translated_text_about = self.base().tr("about_software");
            btn.set_text(translated_text_about.to_godot());
            let mut menu_popup_about = btn.get_popup().unwrap();
            menu_popup_about.clear();
            let translated_text_version_information = self.base().tr("version_information");
            menu_popup_about.add_item(translated_text_version_information.to_godot());
            menu_popup_about.set_item_id(0, 3001);
            menu_popup_about.add_separator();           // 分割线
            let translated_text_instructions_for_use = self.base().tr("instructions_for_use");
            menu_popup_about.add_item(translated_text_instructions_for_use.to_godot());
            menu_popup_about.set_item_id(2, 3002);
            menu_popup_about.add_separator();           // 分割线
            let translated_text_instructions_help_documentation = self.base().tr("help_documentation");
            menu_popup_about.add_item(translated_text_instructions_help_documentation.to_godot());
            menu_popup_about.set_item_id(4, 3003);
            menu_popup_about.add_separator();           // 分割线
            let translated_text_instructions_contact_the_author = self.base().tr("contact_the_author");
            menu_popup_about.add_item(translated_text_instructions_contact_the_author.to_godot());
            menu_popup_about.set_item_id(6, 3004);
        }


        // 如果按钮还没创建，则创建并添加
        if self.btn_exit.is_none() {
            let mut menu_button_exit = MenuButton::new_alloc();
            menu_button_exit.set_position(Vector2::new(1050.0, 20.0));
            let mut menu_popup_exit = menu_button_exit.get_popup().unwrap();
            menu_popup_exit.connect(
                "id_pressed", 
                &(self.base().callable("on_menu_item_pressed"))
            );
            // 菜单退出被点击
            menu_button_exit.connect(
                "pressed", 
                &(self.base().callable("on_menu_exit_pressed"))
            );
            self.base_mut().add_child(&menu_button_exit);
            // 赋值给配置按钮
            self.btn_exit = Some(menu_button_exit);
        }

        // 无论是否新创建，每次调用都更新文字（处理翻译）
        if let Some(mut btn) = self.btn_exit.clone() {
            let translated_text_exit = self.base().tr("exit_software");
            btn.set_text(translated_text_exit.to_godot());
            let mut menu_popup_exit = btn.get_popup().unwrap();
            menu_popup_exit.clear();
            let translated_text_minimize_window = self.base().tr("minimize_window");
            menu_popup_exit.add_item(translated_text_minimize_window.to_godot());
            menu_popup_exit.set_item_id(0, 4001);
            menu_popup_exit.add_separator();           // 分割线
            let translated_text_exit_program = self.base().tr("exit_program");
            menu_popup_exit.add_item(translated_text_exit_program.to_godot());
            menu_popup_exit.set_item_id(2, 4002);
        }


        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_menu_theme.tres");
        // 应用到主节点，子节点会继承该主题
        self.base_mut().set_theme(&my_theme);
    }


    #[func]
    fn on_menu_item_pressed(&mut self, id: i64) {
        match id {
            1001 => {
                godot_print!("点击了：rust安装路径");
                // 启动文件夹类型的对话框
                let translated_text_select_cargo_path = self.base().tr("select_cargo_path");
                self.open_dir_dialog(translated_text_select_cargo_path.to_string())
            },
            1002 => {
                godot_print!("点击了：Godot启动文件");
                // 启动文件类型的对话框
                let translated_text_select_godot_launcher = self.base().tr("select_godot_launcher");
                self.open_file_dialog(translated_text_select_godot_launcher.to_string())
            },
            1003 => {
                godot_print!("点击了：设置语言");
                // 启动文件类型的对话框
                let scene_path = String::from("res://scene/dialog_setting_language.tscn");
                self.create_dialog_by_scene(scene_path);
            },
            3001 => {
                godot_print!("点击了：rust版本信息");
                self.append_to_scene("about_version".to_string());
            },
             3002 => {
                godot_print!("点击了：使用说明");
                self.append_to_scene("about_instruction".to_string());
            },
            3003 => {
                godot_print!("点击了：rust帮助文档");
                let url = "https://blog.csdn.net/ShiShiLunHui/article/details/159385949?spm=1011.2415.3001.5331".to_string();
                self.open_url(url);
            },
            3004 => {
                godot_print!("点击了：rust联系作者");
                 self.append_to_scene("about_author".to_string());
            },
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
    fn on_menu_config_pressed(&mut self){
         godot_print!("点击了：环境配置");
         self.append_to_scene("main_default".to_string());
    }

    #[func]
    fn on_menu_project_pressed(&mut self){
         godot_print!("点击了：创建项目");
         self.append_to_scene("main_project".to_string());
         // 在这里获取一下存储的数据
         let current_path_godot = SecureStorage::get("path_godot");
         let current_path_rust = SecureStorage::get("path_rust");
         godot_print!("path_godot 路径: {}", current_path_godot);
         godot_print!("path_rust 路径: {}", current_path_rust);
    }

    #[func]
    fn on_menu_about_pressed(&mut self){
         godot_print!("点击了：关于软件");
         self.append_to_scene("main_default".to_string());
    }

    #[func]
    fn on_menu_exit_pressed(&mut self){
         godot_print!("点击了：退出软件");
         self.append_to_scene("main_default".to_string());
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



    // 打开文件夹的对话框
    #[func]
    fn open_dir_dialog(&mut self, &title: String){
        // 1. 动态实例化 使用新的 API 名称：from_init_fn
        let mut dialog = Gd::<MyFileDialog>::from_init_fn( |base|{
            MyFileDialog { base, kind:"path_rust".to_string() }
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


    // 打开文件的对话框
    #[func]
    fn open_file_dialog(&mut self, &title: String){
        // 1. 动态实例化 使用新的 API 名称：from_init_fn
        let mut dialog = Gd::<MyFileDialog>::from_init_fn( |base|{
            MyFileDialog { base, kind:"path_godot".to_string() }
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
        dialog.set_title(&title);
        dialog.set_use_native_dialog(true); 

        let docs_path = godot::classes::Os::singleton().get_system_dir(SystemDir::DOCUMENTS);
        dialog.set_current_dir(&docs_path);
        // 4. 必须先加入场景树
        let dialog_node = dialog.clone().upcast::<Node>();
        self.base_mut().add_child(&dialog_node);
        // 5. 弹出
        dialog.bind_mut().open_dialog();
    }


    #[func]
    pub fn open_url(&self, url: String) {
        // 1. 获取 OS 单例
        let mut os = Os::singleton();
        // 2. 调用 shell_open
        // 注意：shell_open 接收的是 GString
        os.shell_open(&url);
        godot_print!("正在尝试打开网址...");
    }


    // 添加场景到当前场景中
    #[func]
    fn append_to_scene(&mut self, scene_name: String) {
        let path = format!("res://scene/{}.tscn", scene_name);
        
        // 1. 加载场景资源
        let pack_scene = load::<PackedScene>(&path);
        // 2. 实例化场景 （返回的是 Option<Gd<Node>>）
        if let Some(new_scene) = pack_scene.instantiate(){
            // 3. 将新节点添加为当前节点的子节点
            self.base_mut().add_child(&new_scene)
        }
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


    // 切换整个场景
    #[func]
    fn change_to_scene(&mut self, scene_name: String) {
        let path = format!("res://scene/{}.tscn", scene_name);
        
        // 1. 加载场景资源
        let new_scene = load::<PackedScene>(&path);
        // 2. 获取Tree 并且切换
        if let Some(mut tree) = self.base().get_tree(){
            tree.change_scene_to_packed(&new_scene);
        }
    }

    // 改变语言
    #[func]
    fn on_change_language(&mut self, text: GString){
        godot_print!("显示内容 ....on_change_language {text}");
        self.on_ready();
    }
}