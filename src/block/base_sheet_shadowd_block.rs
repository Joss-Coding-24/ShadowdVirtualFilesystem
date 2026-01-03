use crate::{
    algoritm::cursors::Cursor, 
    block::{
        allocator_block::AllocatorBlock, 
        insert_helpers::{
            BufferStates, InsertResult, InsertResultItem, TransitOptions, TransitReturn
        }, 
        shadowd_block::{
            Block, 
            ShadowdBlockCore
        }
    }, 
    helpers::convertions::size_to_big
};

pub struct BaseSheetShadowdBlock<'a>{
    _core:ShadowdBlockCore<'a>,
    buffer:Vec<u8>,
    is_free:bool,
    writed_bytes:usize,
    free_bytes:usize,
    start: u64,
    size: usize,
    head_size: u8,
    data:usize
}

//la politica de general de bloque
impl <'a> Block<'a> for BaseSheetShadowdBlock<'a> {
    type Buffer = Vec<u8>;

    fn new(pos:u64, alloc:&'a mut AllocatorBlock, disk_id:usize)->Self {
        let instace = BaseSheetShadowdBlock {
            _core:
                ShadowdBlockCore{
                    pos, 
                    disk_id, 
                    alloc
                },
            
            buffer:
                Vec::new(), 
            is_free: 
                true, 
            writed_bytes: 
                0, 
            free_bytes: 
                AllocatorBlock::get_size_block(), 
            start: 
                AllocatorBlock::get_size_block() as u64 *pos,
            size: 
                AllocatorBlock::get_size_block(),
            head_size: 
                4,
            data:
                AllocatorBlock::get_size_block()-4
        };
        instace.read_intern();
        instace
    }
    fn write_intern(&mut self) {
        let buffer_size = self.buffer.len();
        let to_write = if buffer_size < self.data{buffer_size}else{self.data};
        let mut count = self.start;

        let count_writed = if self.writed_bytes < self.data {self.writed_bytes}else{self.data};
        let header_bytes = size_to_big(count_writed, self.head_size as u16);
        let alloc:&mut AllocatorBlock =  self._core.alloc;
        let result = &alloc.write_disk(&header_bytes, self.head_size as usize, count, self._core.disk_id as u16);
        if result.is_none() {
            return
        }
        count += self.head_size as u64;

        let data_bytes= &mut self.buffer;
        data_bytes.resize(to_write, 0);
        alloc.write_disk(data_bytes, to_write, count, self._core.disk_id as u16);
        return
    }
    fn write_block(&self, cur:&Cursor, data:&mut Vec<u8>) -> InsertResult{
        if self.free_bytes == 0{
            return InsertResult{
                result:
                    InsertResultItem::BufferIsFull,
                state:
                    BufferStates::Full,
                remaining:
                    data.len(),
                written:
                    0,
                remaining_bytes:
                    Vec::from(data)
            }
        }
        let remaning = self.free_bytes;

    }
    fn read_to(&self, cur:&Cursor, size:usize) -> &[u8]{
        todo!("readTo")
    }
    fn clear_block_childs(&self)->bool {
        todo!("Clear")
    }
    fn insert_to(&self, options:&TransitOptions)->TransitReturn {
        todo!("insert")
    }
    fn remove_to(&self, options:&TransitOptions)->TransitReturn {
        todo!("remove")
    }
    fn to_string(&self) -> String {
        todo!("toString")
    }
}

//la politica interna del bloque 
impl BaseSheetShadowdBlock<'_> {
    pub fn layer()->u8{
        1
    }
    pub fn is_free(&self) -> bool{
        self.is_free
    }
    pub fn writed_bytes(&self) -> usize{
        self.writed_bytes
    }
    pub fn free_bytes(&self) -> usize{
        self.free_bytes
    }
    fn read_intern(&self){
        todo!("read")
    }
}

pub struct EntrySheetShadowdBlock<'a>{
    pos:usize,
    bSSB:BaseSheetShadowdBlock<'a>,
    valid:bool,
}