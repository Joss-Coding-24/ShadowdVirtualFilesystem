use std::{
    cell::RefCell,
    rc::Rc};
use crate::block::allocator_block::AllocatorBlock;

pub mod allocator_block;
pub mod shadowd_block;
pub mod base_sheet_shadowd_block;
pub mod insert_helpers;
mod disk_helper;
pub type AllocHadle = Rc<RefCell<AllocatorBlock>>;