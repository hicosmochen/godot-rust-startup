use godot::prelude::*;
use godot::classes::{Button, IButton, Theme}; // 导入需要的 UI 类


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
}


#[godot_api]
impl IButton for ProjectButton {
    fn init(base: Base<Button>) -> Self {
        godot_print!("按钮"); 
        Self {
            base,
            kind: ButtonKind::SelectWorkSpace,   // 默认初始化
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
    fn on_button_pressed(&self){
         // 根据 kind 执行逻辑
        match self.kind {
            ButtonKind::SelectWorkSpace => godot_print!("按钮被点击了...SelectWorkSpace"),
            ButtonKind::RustRootPath => godot_print!("按钮被点击了...RustRootPath"),
            ButtonKind::GdextFileName => godot_print!("按钮被点击了...GdextFileName"),
            ButtonKind::NeedCreateDemo => godot_print!("按钮被点击了...NeedCreateDemo"),
            ButtonKind::StartCreatProject => godot_print!("按钮被点击了...StartCreatProject"),
        } 
    }
}


