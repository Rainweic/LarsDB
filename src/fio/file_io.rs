use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::prelude::FileExt;
use std::sync::Arc;
use parking_lot::RwLock;
use log::error;
use crate::fio::IOManager;

pub struct FileIO {
    fd: Arc<RwLock<File>>,
}

impl FileIO {
    pub fn new(file_name: &str) -> Self {
        let fd = OpenOptions::new()
                        .create(true)
                        .read(true)
                        .write(true)
                        .append(true)
                        .open(file_name)
                        .expect("打开文件错误");
        Self {
            fd: Arc::new(RwLock::new(fd))
        }
    }
}

impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, Err> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("读取文件错误：{e}");
                return Err(e);
            }
        };
    }

    fn write(&self, buf: &[u8]) -> Result<usize, Err> {
        let mut write_gurad = self.fd.write();
        match write_gurad.write(buf) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("写入文件错误: {e}");
                return Err(e);
            }
        }
    }

    fn sync(&self) -> Result<(), Err> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("sync失败: {e}");
            return Err(e)
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_file_io_write() {

    }
}
