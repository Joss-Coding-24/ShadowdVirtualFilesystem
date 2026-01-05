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
use std::mem::take;

pub struct BaseSheetShadowdBlock<'a>{
    _core:ShadowdBlockCore<'a>,
    _buffer:Vec<u8>,
    is_free:bool,
    writed_bytes:usize,
    free_bytes:usize,
    _start: u64,
    _size: usize,
    _head_size: u8,
    _data:usize
}

//la politica de general de bloque
impl <'a> Block<'a> for BaseSheetShadowdBlock<'a> {
    type Buffer = Vec<u8>;

    fn new(pos:u64, alloc:&'a mut AllocatorBlock, disk_id:usize)->Self {
        let instace:BaseSheetShadowdBlock<'a> = BaseSheetShadowdBlock {
            _core:
                ShadowdBlockCore{
                    pos, 
                    disk_id, 
                    alloc
                },
            
            _buffer:
                Vec::new(),
            is_free: 
                true, 
            writed_bytes: 
                0, 
            free_bytes: 
                AllocatorBlock::get_size_block(), 
            _start: 
                AllocatorBlock::get_size_block() as u64 *pos,
            _size: 
                AllocatorBlock::get_size_block(),
            _head_size: 
                4,
            _data:
                AllocatorBlock::get_size_block()-4
        };
        instace.read_intern();
        instace
    }
    fn write_intern(&mut self) {
        let buffer_size = self._buffer.len();
        let to_write = if buffer_size < self._data{buffer_size}else{self._data};
        let mut count = self._start;

        let count_writed = if self.writed_bytes < self._data {self.writed_bytes}else{self._data};
        let header_bytes = size_to_big(count_writed, self._head_size as u16);
        let alloc:&mut AllocatorBlock =  self._core.alloc;
        let result = &alloc.write_disk(&header_bytes, self._head_size as usize, count, self._core.disk_id as u16);
        if result.is_none() {
            return
        }
        count += self._head_size as u64;

        let data_bytes= &mut self._buffer;
        data_bytes.resize(to_write, 0);
        alloc.write_disk(data_bytes, to_write, count, self._core.disk_id as u16);
        return
    }
    fn write_block(&mut self, cur:&Cursor, data:&mut Vec<u8>) -> InsertResult{
        if self.free_bytes == 0 {
            return InsertResult {
                result: InsertResultItem::BufferIsFull,
                state: BufferStates::Full,
                remaining: data.len(),
                written: 0,
                remaining_bytes: take(data),
            };
        }
        let original_data_size = data.len();
        let remaning = self.free_bytes;
        let to_write = if data.len() < remaning {
            data.len()
        }else{
            remaning
        };
        let mut data_remaning_bytes = if to_write < data.len() {
            data.split_off(to_write)// movemos lo que no cabe 
        }else{
            Vec::new()
        };

        self._buffer.append(data);

        let remaning_bytes = if self._buffer.len() > self._data {
            let mut vec = self._buffer.split_off(self._data);
            vec.append(&mut data_remaning_bytes);
            vec
        }else{
            data_remaning_bytes
        }; // por seguridad

        self.writed_bytes += to_write;
        self.free_bytes -= to_write;

        let is_fully = if self.free_bytes == 0 {
            self.write_intern();
            true
        } else {
            false
        };
        let mut result = InsertResult {
            result: InsertResultItem::Fail,
            state: BufferStates::Empty,
            remaining: remaning_bytes.len(),
            written: 0,
            remaining_bytes: remaning_bytes,
        };

        result.written = to_write;

        result.state = if is_fully {
            BufferStates::Full
        } else if self.free_bytes == self._data {
            BufferStates::Empty
        } else {
            BufferStates::PartiallyFull
        };

        result.result = if to_write == original_data_size {
            InsertResultItem::InsertedWithoutRemaining
        } else {
            InsertResultItem::InsertedWithRemaining
        };

        result

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