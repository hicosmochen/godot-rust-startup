use godot::prelude::*;
use godot::classes::{Button, IButton}; // 导入需要的 UI 类

use godot::classes::TranslationServer;
use crate::secure::secure_storage::SecureStorage;


// 定义 Button 的枚举类型, 让godot可以进行外部赋值（赋值的内容是指定的数据值）
#[derive(GodotConvert, Var, Export, Default, Copy, Clone, Debug)]
// 告诉 Godot 以整数的形式存储索引
#[godot(via = i64)]
pub enum ButtonKind {
    #[default]
    EnUs,
    ZhCn,
    ZhTw,
    JaJp,
    KoKr,
    DeDe,
    FrFr,
    EsEs,
    ItIt,
}


#[derive(GodotClass)]
#[class(base=Button)] 
pub struct LanguageButton {

    pub base: Base<Button>,

    #[export]
    pub kind: ButtonKind,
}

#[godot_api]
impl IButton for LanguageButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            kind: ButtonKind::EnUs,   // 默认初始化
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
impl LanguageButton {

    #[func]
    fn on_button_pressed(&mut self){
         // 根据 kind 执行逻辑
        match self.kind {
            ButtonKind::EnUs => {
                godot_print!("点击了： EnUs");
                self.change_language(format!("en_US"));
                self.close_project_dialog();
            },
            ButtonKind::ZhCn => {
                godot_print!("点击了： ZhCn");
                self.change_language(format!("zh_CN"));
                self.close_project_dialog();
            },
            ButtonKind::ZhTw => {
                godot_print!("点击了： ZhTw");
                self.change_language(format!("zh_TW"));
                self.close_project_dialog();
            },
            ButtonKind::JaJp => {
                godot_print!("点击了： JaJp");
                self.change_language(format!("ja_JP"));
                self.close_project_dialog();
            },
            ButtonKind::KoKr => {
                godot_print!("点击了： KoKr");
                self.change_language(format!("ko_KR"));
                self.close_project_dialog();
            },
            ButtonKind::DeDe => {
                godot_print!("点击了： DeDe");
                self.change_language(format!("de_DE"));
                self.close_project_dialog();
            },
            ButtonKind::FrFr => {
                godot_print!("点击了： FrFr");
                self.change_language(format!("fr_FR"));
                self.close_project_dialog();
            },
            ButtonKind::EsEs => {
                godot_print!("点击了： EsEs");
                self.change_language(format!("es_ES"));
                self.close_project_dialog();
            },
            ButtonKind::ItIt => {
                godot_print!("点击了： ItIt");
                self.change_language(format!("it_IT"));
                self.close_project_dialog();
            },
        }
    }


    // 更改语言
    #[func]
    fn change_language(&mut self, language_code : String){
        
        // 获取单例
        let mut server = TranslationServer::singleton();
        // 转换为 Godot 的字符串类型并且设置
        server.set_locale(&language_code);
        // 存储当前设置的语言
        SecureStorage::save("current_lanague", &language_code);
        // 打印日志, 确定设置成功
        godot_print!("语言已经切换至1: {}" , server.get_locale());
        godot_print!("语言已经切换至x: {}" , SecureStorage::get("current_lanague"));

        // 发送信号
        self.base().get_tree().unwrap().call_group("listener_change_language", "on_change_language",  &[language_code.to_variant()]);
    }




    // 关闭对话框的场景
    #[func]
    fn close_project_dialog(&mut self){
        // 关闭当前的场景
        let mut tree = self.base().get_tree().unwrap();
        // 找到组内所有成员并销毁
        tree.call_group("node_dialog", "queue_free", &[]);
    }
}