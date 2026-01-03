use std::{
    i64, u8, usize
};

use crate::block::disk_helper::DiskHelper;
struct DiskHelperManage {
    fragments: [Option<DiskHelper>; 4],
}
impl DiskHelperManage {
    fn new() -> Self {
        Self {
            fragments: [None, None, None, None],
        }
    }
    fn get_or_create(
        &mut self,
        disk: usize,
        frag: u64,
    ) -> Option<&mut DiskHelper> {
        let idx = frag_index(frag)?;

        if self.fragments[idx].is_none() {
            let name = format!("{}.disk{}", disk, frag);
            self.fragments[idx] = Some(DiskHelper::new(&name));
        }

        self.fragments[idx].as_mut()
    }

}
pub struct AllocatorBlock{
    helpers:Vec<DiskHelperManage>,
}

impl AllocatorBlock{
    pub fn new() -> Self {
        Self {
            helpers: (0..u16::MAX).map(|_| DiskHelperManage::new()).collect(),
        }
    }

    pub fn get(&self, pos:usize){
        todo!("get sin implementar")
    }
    pub fn get_size_block()->usize{
        todo!("get size sin imolementar")
    }
    pub fn write_disk(
        &mut self,
        data: &[u8],
        count: usize,
        off: u64,
        disk_id: u16,
    ) -> Option<()> {
        let max = i64::MAX as u64;
        let frag = off / max;
        let inner_off = off % max;

        let helper = self.helpers
            .get_mut(disk_id as usize)?
            .get_or_create(disk_id as usize, frag)?;

        helper.write_at(data, count, inner_off as i64);
        Some(())
    }
    pub fn read_disk(
        &mut self,
        count: usize,
        off: u64,
        disk_id: u16,
    ) -> Option<Vec<u8>> {
        let max = i64::MAX as u64;
        let frag = off / max;
        let inner_off = off % max;

        let helper = self.helpers
            .get_mut(disk_id as usize)?
            .get_or_create(disk_id as usize, frag)?;

        Some(helper.read_at(count, inner_off as i64))
    }
}
fn make_option(disk:usize, frag:u8) -> DiskHelper{
    let disk_name= format!("{}.disk{}", disk, frag);
    DiskHelper::new(&disk_name)
}
fn frag_index(frg: u64) -> Option<usize> {
    match frg {
        0 => None,           // inválido
        1..=4 => Some((frg - 1) as usize),
        _ => None,
    }
}