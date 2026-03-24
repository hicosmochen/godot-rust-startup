use serde::{Serialize, Deserialize};

// 定义了结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct SecureData {
    pub path_godot: String,
    pub path_rust: String,
    pub first_launch_ms: u64,
    pub password_hash: String,
}


// 实现默认值
impl Default for SecureData {
    fn default() -> Self {
        Self {
            path_godot: "user://config/godot".to_string(),
            path_rust: "user://config/rust".to_string(),
            first_launch_ms: 1711234567890,
            password_hash: "default_secret".to_string(),
        }
    }
}