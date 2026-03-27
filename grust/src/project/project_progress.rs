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
        // 初始化进度
        self.base_mut().set_value(0.0);
    }
}

// #[func] 必须放在单独的 impl 块中
#[godot_api]
impl ProjectProgressBar {
    // 设置进度值
    #[func]
    pub fn set_progress(&mut self, amount: f64) {
        // 进行边界检查或逻辑处理
        let clamped_amount = amount.clamp(0.0, 100.0);
        
        // 调用基类 ProgressBar 的方法
        self.base_mut().set_value(clamped_amount);
        
        godot_print!("进度已更新为: {}", clamped_amount);
    }

    // 对外暴露的增加进度方法
    #[func]
    pub fn add_progress(&mut self, delta: f64) {
        let current = self.base().get_value();
        let new_val = current + delta;
        self.base_mut().set_value(new_val);
    }

    // 获取当前值
    #[func]
    pub fn get_current_value(&self) -> f64 {
        self.base().get_value()
    }
}