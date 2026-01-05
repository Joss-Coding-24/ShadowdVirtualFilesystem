use std::{
    u8, 
    usize
};

use crate::{
    algoritm::cursors::Cursor, 
    block::{
        allocator_block::AllocatorBlock, 
        insert_helpers::{
            InsertResult, 
            TransitOptions, 
            TransitReturn
        }
    }
};

pub struct ShadowdBlockCore<'a> {
    pub pos: u64,
    pub disk_id: usize,
    pub alloc: &'a mut AllocatorBlock,
}

pub trait Block <'a> {
    type Buffer;

    fn new(pos:u64, alloc:&'a mut AllocatorBlock, disk_id:usize)->Self;
    fn write_intern(&mut self);
    fn write_block(&mut self, cur:&Cursor, data:&mut Vec<u8>)->InsertResult;
    fn read_to(&self, cur:&Cursor, size:usize)->&[u8];
    fn clear_block_childs(&self)->bool;
    fn remove_to(&self, options:&TransitOptions)->TransitReturn;
    fn insert_to(&self, options:&TransitOptions)->TransitReturn;
    fn to_string(&self) -> String;
    
}