use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

use serde::{Serialize, Deserialize};
use csv::{ReaderBuilder, WriterBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    TODO,
    IN_PROGRESS,
    DONE,
    ELSE
}

pub struct Database {
    pub file: File,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            Status::TODO => "TODO",
            Status::IN_PROGRESS => "IN_PROGRESS",
            Status::DONE => "DONE",
            Status::ELSE => "ELSE",
        };
        write!(f, "{}", status_str)
    }
}

impl Database {
    /// 打开指定路径的数据库文件并返回一个 [`Database`] 实例。
    ///
    /// 如果文件不存在，将自动创建；
    /// 如果没有访问权限，则返回错误。
    ///
    /// # 参数
    ///
    /// - `filename`：数据库文件的路径。
    ///
    /// # 返回值
    ///
    /// 返回一个 [`Database`] 实例，内部持有打开的文件句柄。
    /// 如果打开失败，返回 [`io::Error`]。
    ///
    /// # 示例
    ///
    /// ```
    /// let db = Database::open(".rododb").expect("Failed to open database file");
    /// ```
    ///
    /// [`Database`]: struct.Database.html
    /// [`io::Error`]: io::Error
    pub fn open(filename: &str) -> io::Result<Database> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filename)?;

        Ok(Database { file })
    }

    /// 向数据库中添加一条任务记录。
    ///
    /// 该函数将任务信息以一行 CSV 格式写入数据库文件末尾，
    /// 格式为：`id,content,status\n`。
    ///
    /// # 参数
    ///
    /// - `task`：要添加的任务结构体，包含任务 ID、内容和状态。
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`。
    /// 如果写入数据库文件失败，则返回 [`io::Error`]。
    ///
    /// # 示例
    ///
    /// ```
    /// let task = Task {
    ///     id: 1,
    ///     content: "Buy milk".to_string(),
    ///     status: Status::TODO,
    /// };
    /// db.add_task(task).expect("Failed to add task");
    /// ```
    ///
    /// [`io::Error`]: io::Error

    pub fn add_task(&mut self, task: Task) ->  io::Result<()> {
        let content = &task.content.clone();
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(&mut self.file);
        writer.serialize(task)?;
        writer.flush()?; // 确保写入磁盘
        println!("\tItem added: {}", content);
        Ok(())
    }

    pub fn parse_task_line(line: &str) -> Option<Task> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let mut records = reader.deserialize();
        if let Some(result) = records.next() {
            result.ok()
        } else {
            None
        }
    }
}
