use std::{mem::take, sync::{Arc, RwLock}};

use crate::{algoritm::cursors::Cursor, block::{AllocHadle, base_sheet_shadowd_block::{BaseSheetShadowdBlock, EntrySheetShadowdBlock}, disk_helper, insert_helpers::{BufferStates, InsertResult, InsertResultItem, TransitOptions, TransitReturn}, shadowd_block::Block}};

struct BaseBranchShadowdBlock1{
    root:BaseSheetShadowdBlock,
    childs:Arc<RwLock<Vec<EntrySheetShadowdBlock>>>,
    write_count:usize,
    salve_count:usize,
}

impl Block for BaseBranchShadowdBlock1 {
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
            let buff = &mut self.childs.write().ok()?;
            for i in self.write_count..buff.len(){
                let v = &mut buff[i];
                if v.is_valid(){
                    let block =v.get_bs().as_mut()?;
                    block.write_intern();
                }else{
                    self.salve_count = i;
                }
            };
            Some(())
        }
    }
    
    fn write_block(&mut self, data:&mut Vec<u8>)->Option<InsertResult> {
        if self.write_count == 31{
            return Some(InsertResult{
                result: InsertResultItem::BufferIsFull,
                state: BufferStates::Full,
                remaining: data.len(),
                written: 0,
                remaining_bytes: take(data)
            })
        }
        let mut buff = self.childs.write().ok()?;
        let mut bytes = take(data);
        for i in self.write_count..31{
            if buff.len() <= i {
                self.gen_child(i);
            }
            let mut block = &mut buff[i];
            if !block.is_valid() {
                self.gen_child(i);
                block = &mut buff[i];
            }        
            let size = bytes.len();
            let bs = block.get_bs().as_mut()?;
            let result = bs.write_block(data)?;
            
            if result.remaining + result.written == size {
                self.write_count += 1;
                bytes = take(&mut result.remaining_bytes.clone())// aca se destruye igual, el clone es solo para poner el &mut
            }else{
                break;
            }

            if bytes.len() == 0 {
                break;
            }
        };
        None
    }
    
    fn read_to(&mut self, cur: &mut Cursor, size: usize) -> Option<Vec<u8>> {
        let mut buff = Vec::new();
        let mut cout = 0;
        
        loop {
            if cout == size {
                break ;
            };

            let actual = cur.get_pos(1)?;
            let childs = &mut self.childs.read().ok()?;
                let block: &mut EntrySheetShadowdBlock = childs[actual];
                if block.is_valid(){
                    let to_read = size-cout;
                    if let chunk = block.bs_sb?.read_to(cur, to_read)?;
                    let sized = chunk.len();
                    if sized == 0 {
                        break
                    }
                    cout += sized;
                    buff.extend_from_slice(chunk.as_slice());
                }
        };
        Some(buff)
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

impl BaseBranchShadowdBlock1 {
    fn read_intern(&mut self){}
    fn gen_child(&self, pos:usize){}
}