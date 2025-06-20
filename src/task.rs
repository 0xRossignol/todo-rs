use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    TODO,
    IN_PROGRESS,
    DONE,
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

    pub fn add_task(&mut self, task: Task) ->  io::Result<()> {
        let line = format!("{},{},{}\n", task.id,task.content, task.status);
        writeln!(self.file, "{}", &line)?;
        println!("\tItem added: {}", task.content);
        Ok(())
    }
}
