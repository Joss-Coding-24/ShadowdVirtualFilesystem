use std::usize;

pub struct RandomAccessFile{

}

impl RandomAccessFile {
    pub fn new(path:&str)->Self{
        todo!("new de raf esta sin implementar")
    }
    pub fn write_at(&self, data:&[u8], count:usize, offset:i64){
        todo!("wtite de raf esta sin implementar")
    }
    pub fn read_at(&self, count:usize, offset:i64) -> Vec<u8>{
        todo!("read de raf esta sin implenentart")
    }
    pub fn size(&self)->u64{
        todo!("size de raf sin imolementar")
    }
}