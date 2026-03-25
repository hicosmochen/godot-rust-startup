use godot::prelude::*;
use godot::classes::{ProgressBar, IProgressBar}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=ProgressBar)] 
pub struct ProjectProgressBar {
    base: Base<ProgressBar>
}


#[godot_api]
impl IProgressBar for ProjectProgressBar {
    fn init(base: Base<ProgressBar>) -> Self {
        godot_print!("进度条"); 
        Self {base}
    }

    fn ready(&mut self) {
        
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectProgressBar {

}