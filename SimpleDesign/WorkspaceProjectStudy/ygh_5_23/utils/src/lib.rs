//! 工具库
//! 提供各种通用工具函数，如ID生成和日期格式化


// ID生成器模块
pub mod id_gen {
    use rand::Rng;

    // 生成4位随机字符串ID
    pub fn generate_id() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        (0..4)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

// 日期处理模块
pub mod date {
    use chrono::Local;

    // 格式化当前日期(YYYY-MM-DD)
    pub(crate) fn format_date() -> String {
        let now = Local::now();
        now.format("%Y-%m-%d").to_string()
    }
}


// 测试模块
#[cfg(test)]
mod tests {
    use super::{id_gen, date};

    #[test]
    fn test_generate_id() {
        let id = id_gen::generate_id();
        assert_eq!(id.len(), 4);
        assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_format_date() {
        let date = date::format_date();
        assert_eq!(date.len(), 10);
        assert!(date.contains('-'));
    }
}
