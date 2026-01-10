use std::{
    path::{
        Path, 
        PathBuf
    }
};

use crate::block::{
    disk_helper::DiskHelper
};
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
    fn mix64(mut x: u64) -> u64 {
        x ^= x >> 33;
        x = x.wrapping_mul(0xff51afd7ed558ccd);
        x ^= x >> 33;
        x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
        x ^= x >> 33;
        x
    }
    pub fn get_layer_key(layer: u64) -> u64 {
        Self::mix64(layer.wrapping_mul(0x9E3779B97F4A7C15))
    }
    pub fn get_disk_key(layer_key: u64, disk: u64) -> u64 {
        let dk = Self::mix64(disk.wrapping_mul(0x8D2668A86E396B04));
        layer_key ^ dk
    }
    pub fn get_block_key(disk_key: u64, block_id: u64) -> u64 {
        let bk = Self::mix64(block_id.wrapping_mul(0x7C1557975D2859F3));
        disk_key ^ bk
    }
    pub fn get_ofusc_key(layer:u64, disk:u64, block:u64)->u64{
        let layer_key = Self::get_layer_key(layer);
        let disk_key = Self::get_disk_key(layer_key, disk);
        Self::get_block_key(disk_key, block)
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
fn frag_index(frg: u64) -> Option<usize> {
    match frg {
        0..=3 => Some((frg) as usize),
        _ => None,
    }
}