use std::{
    cell::RefCell, i64, path::{Path, PathBuf}, rc::Rc, u8, usize
};

use crate::block::{AllocHadle, base_sheet_shadowd_block::BaseSheetShadowdBlock, disk_helper::DiskHelper, shadowd_block::Block};
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
        dir:PathBuf
    ) -> Option<&mut DiskHelper> {
        let idx = frag_index(frag)?;

        if self.fragments[idx].is_none() {
            let mut path = dir.clone();
            path.push(format!("{}.disk{}", disk, frag));
            self.fragments[idx] = Some(DiskHelper::new(&path)?);
        }

        self.fragments[idx].as_mut()
    }

}
pub struct AllocatorBlock{
    helpers:Vec<DiskHelperManage>,
    dir:PathBuf
}

impl AllocatorBlock{
    pub fn new(dir:impl AsRef<Path>) -> Self{
        AllocatorBlock {
            helpers: (0..u16::MAX).map(|_| DiskHelperManage::new()).collect(),
            dir:dir.as_ref().to_path_buf()
        }
    }

    pub fn get_bssb(self, pos:u64, disk_id:usize)->BaseSheetShadowdBlock{
        BaseSheetShadowdBlock::new(pos, Rc::new(RefCell::new(self)), disk_id)
    }
    pub fn get_size_block()->usize{
        341
    }
    pub fn write_disk(
        &mut self,
        data: &[u8],
        off: u64,
        disk_id: u16,
    ) -> Option<()> {
        let max = i64::MAX as u64;
        let frag = off / max;
        let inner_off = off % max;

        let helper = self.helpers
            .get_mut(disk_id as usize)?
            .get_or_create(disk_id as usize, frag, self.dir.clone())?;

        helper.write_at(data, inner_off as u64)?;
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
            .get_or_create(disk_id as usize, frag, self.dir.clone())?;

        Some(helper.read_at(count, inner_off as u64)?)
    }
}
fn make_option(disk:usize, frag:u8) -> Option<DiskHelper>{
    let disk_name= format!("{}.disk{}", disk, frag);
    Some(DiskHelper::new(&disk_name)?)
}
fn frag_index(frg: u64) -> Option<usize> {
    match frg {
        0 => None,           // inválido
        1..=4 => Some((frg - 1) as usize),
        _ => None,
    }
}