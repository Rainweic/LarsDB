pub mod file_io;

pub trait IOManager: Sync + Send {

    /// 从文件的指定位置读取对应的数据
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, Err>;

    /// 写入字节数组到文件中
    fn write(&self, buf: &[u8]) -> Result<usize, Err>;

    /// 持久化文件
    fn sync(&self) -> Result<(), Err>;

}