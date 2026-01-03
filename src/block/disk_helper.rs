use crate::helpers::random_access_file::RandomAccessFile;

pub struct DiskHelper{
    pub raf:RandomAccessFile,
}

impl DiskHelper{
    pub fn new(disk:&str)->Self{
        DiskHelper{
            raf:
                RandomAccessFile::new(disk)
        }
    }
    pub fn write_at(&self, data:&[u8], count:usize, offset:i64){
        self.raf.write_at(data, count, offset);
    }
    pub fn read_at(&self, count:usize, offset:i64) -> Vec<u8>{
        self.raf.read_at(count, offset)
    }
}