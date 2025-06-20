use std::fs::{File, OpenOptions};
use std::process;

pub struct Task {
    pub id: u32,
    pub content: String,
    pub status: Status,
}

pub enum Status {
    TODO,
    IN_PROGRESS,
    DONE,
}

pub struct Database {
    pub file: File,
}

impl Database {
    /// 打开指定路径的数据库文件并返回 `Database` 实例。
    ///
    /// 如果文件无法打开（例如文件不存在或权限不足），
    /// 将输出错误信息并终止程序。
    ///
    /// # 参数
    ///
    /// - `filename`：要打开的文件路径。
    ///
    /// # 返回值
    ///
    /// 返回一个持有文件句柄的 `Database` 实例。
    ///
    /// # 示例
    ///
    /// ```
    /// let db = Database::open(".rodorc");
    /// ```
    /// 
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
            .read(true)
            .open(filename)
            .unwrap_or_else(|e| {
                eprintln!("Could not open database file '{}': {}", filename, e);
                process::exit(1);
            });

        Database { file }
    }
}
