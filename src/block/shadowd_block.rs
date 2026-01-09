use std::{
    u8, 
    usize
};

use crate::{
    algoritm::cursors::Cursor, 
    block::{
        AllocHadle, 
        insert_helpers::{
            InsertResult, 
            TransitOptions, 
            TransitReturn
        }
    }
};

pub struct ShadowdBlockCore {
    pub pos: u64,
    pub disk_id: usize,
    pub alloc: AllocHadle,
}

pub trait Block{
    type Buffer;

    fn new(pos:u64, alloc:AllocHadle, disk_id:usize)->Self;
    fn write_intern(&mut self) -> Option<()>;
    fn write_block(&mut self, cur:&Cursor, data:&mut Vec<u8>)->Option<InsertResult>;
    fn read_to(&mut self, cur:&Cursor, size:usize)-> Option<&[u8]>;
    fn clear_block_childs(&mut self)->bool;
    fn remove_to(&mut self, options:&TransitOptions)->Option<TransitReturn>;
    fn insert_to(&mut self, options:&TransitOptions)->Option<TransitReturn>;
}