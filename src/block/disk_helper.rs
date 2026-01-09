use std::path::Path;

use crate::helpers::random_access_file::RandomAccessFile;

pub struct DiskHelper{
    pub raf:RandomAccessFile,
}

impl DiskHelper{
    pub fn new(disk:&impl  AsRef<Path>)->Option<Self>{
        let raf = RandomAccessFile::new(disk).ok()?;
        Some(DiskHelper{
            raf: raf
                
        })
    }
    pub fn write_at(&self, data:&[u8], offset:u64) -> Option<()>{
        self.raf.write_at(data, offset).ok()?;
        Some(())
    }
    pub fn read_at(&self, count:usize, offset:u64) -> Option<Vec<u8>>{
        Some(self.raf.read_at(count, offset).ok()?)
    }
}