use std::sync::{Arc, RwLock};

use crate::{algoritm::cursors::Cursor, block::{AllocHadle, base_sheet_shadowd_block::{BaseSheetShadowdBlock, EntrySheetShadowdBlock}, disk_helper, insert_helpers::{InsertResult, TransitOptions, TransitReturn}, shadowd_block::Block}};

struct BaseBranchShadowdBlock_1{
    root:BaseSheetShadowdBlock,
    childs:Arc<RwLock<Vec<EntrySheetShadowdBlock>>>,
    write_count:usize,
    salve_count:usize,
}

impl Block for BaseBranchShadowdBlock_1 {
    type Buffer = Vec<EntrySheetShadowdBlock>;

    fn new(pos:u64, alloc:AllocHadle, disk_id: usize, layer: u8
    ) -> Self{
        let root = BaseSheetShadowdBlock::new(pos, alloc, disk_id, layer);
        Self {
            root, 
            write_count: 0,
            salve_count:0,
            childs: Arc::new(RwLock::new(Vec::new()))
        }
    }
    
    fn write_intern(&mut self) -> Option<()> {
        if self.write_count == self.salve_count {
            Some(())
        }else{
            let buff = self.childs.write().ok()?;
            for i in self.write_count..buff.len(){
                let v = buff[i];
                if v.is_valid(){
                    let block = v.bs_sb.unwrap();
                    
                }
            };
        }
    }
    
    fn write_block(&mut self, cur:&Cursor, data:&mut Vec<u8>)->Option<InsertResult> {
        todo!()
    }
    
    fn read_to(&mut self, cur: &Cursor, size: usize) -> Option<Vec<u8>> {
        todo!()
    }
    
    fn clear_block_childs(&mut self)->Option<bool> {
        todo!()
    }
    
    fn remove_to(&mut self, options:&TransitOptions)->Option<TransitReturn> {
        todo!()
    }
    
    fn insert_to(&mut self, options:&TransitOptions)->Option<TransitReturn> {
        todo!()
    }
}

impl BaseBranchShadowdBlock_1 {
    fn read_intern(&mut self){}
}