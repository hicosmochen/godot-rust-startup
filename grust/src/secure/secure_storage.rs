use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use rand::RngCore;
use godot::prelude::*;
use godot::classes::{FileAccess, file_access::ModeFlags};
use crate::secure::secure_data::SecureData;

pub struct SecureStorage;

impl SecureStorage {
    const KEY: &[u8; 32] = b"use_a_32_byte_unique_key_1234567"; // 必须是32字节

    // 绝对路径: C:\Users\Administrator\AppData\Roaming\Godot\app_userdata\godot
    const SAVE_PATH: &'static str = "user://internal_config.bin"; // 存储的路径

    /// 加密结构体并保存到文件
    fn save_data(data: &SecureData, file_path: &str) -> Result<(), String> {
        // 1. 序列化结构体为 JSON 字节
        let json_bytes = serde_json::to_vec(data).map_err(|e| e.to_string())?;

        // 2. 准备加密器
        let cipher = Aes256Gcm::new(Self::KEY.into());
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 3. 加密
        let ciphertext = cipher
            .encrypt(nonce, json_bytes.as_slice())
            .map_err(|_| "Encryption failed".to_string())?;

        // 4. 拼接 Nonce + Ciphertext 并保存
        let mut final_data = nonce_bytes.to_vec();
        final_data.extend(ciphertext);

        let mut file = FileAccess::open(file_path, ModeFlags::WRITE)
            .ok_or("Cannot open file for writing")?;
        file.store_buffer(&PackedByteArray::from_iter(final_data));
        Ok(())
    }

    /// 从文件读取并解密为结构体
    fn load_data(file_path: &str) -> Result<SecureData, String> {
        let file = FileAccess::open(file_path, ModeFlags::READ)
            .ok_or("Cannot open file for reading")?;
        let buffer = file.get_buffer(file.get_length() as i64).to_vec();

        if buffer.len() < 12 { return Err("Invalid file format".to_string()); }

        // 1. 拆分 Nonce 和 密文
        let (nonce_part, cipher_part) = buffer.split_at(12);
        let nonce = Nonce::from_slice(nonce_part);

        // 2. 解密
        let cipher = Aes256Gcm::new(Self::KEY.into());
        let plain_bytes = cipher
            .decrypt(nonce, cipher_part)
            .map_err(|_| "Decryption failed (Key mismatch or data corrupted)".to_string())?;

        // 3. 反序列化
        let data: SecureData = serde_json::from_slice(&plain_bytes).map_err(|e| e.to_string())?;
        
        Ok(data)
    }


    // 存储数据, 以 KEY - VALUE 字符串的形式存储
    pub fn save(key: &str, value: &str) {
        // 修改 save 函数的第一行
        let mut my_data = Self::load_data(Self::SAVE_PATH).unwrap_or_else(|_| {
            godot_print!("配置文件不存在，正在创建新配置...");
            SecureData::default() // 如果加载失败，创建一个全为空或默认值的结构体
        });
          
        match key {
            "path_godot" => my_data.path_godot = value.to_string(),
            "path_rust" => my_data.path_rust = value.to_string(),
            "first_launch_ms" => my_data.first_launch_ms = value.parse().unwrap_or(0),
            "password_hash" => my_data.password_hash = value.to_string(),
            "current_lanague" => my_data.current_lanague = value.to_string(),
            _ => godot_print!("Unknown key: {}", key),
        }

        let _ = Self::save_data(&my_data, Self::SAVE_PATH);
    }

    // 获取数据, 以 KEY 字符串的形式获取 VALUE
    pub fn get(key: &str) -> String {
        let Ok(loaded_data) = Self::load_data(Self::SAVE_PATH) else {
            return "File Error".to_string();
        };

        match key {
            "path_godot" => loaded_data.path_godot,
            "path_rust" => loaded_data.path_rust,
            "first_launch_ms" => loaded_data.first_launch_ms.to_string(),
            "password_hash" => loaded_data.password_hash,
            "current_lanague" => loaded_data.current_lanague,
            _ => "Not Found".to_string(),
        }
    }
}



/*

1、先设置使用的包名称
    use crate::secure::secure_storage::SecureStorage;

2、存放数据的代码
    SecureStorage::save("path_godot", "helloworldxy");


3、获取数据的代码
    let result = SecureStorage::get("path_godot");
    let result = SecureStorage::get("path_rust");

*/