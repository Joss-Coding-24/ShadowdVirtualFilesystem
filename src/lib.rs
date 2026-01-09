pub mod block;
pub mod algoritm;
pub mod helpers;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::algoritm::cursors::Cursor;
    use crate::block::shadowd_block::{Block, ShadowdBlockCore};
    use crate::block::allocator_block::AllocatorBlock;
    use crate::block::base_sheet_shadowd_block::BaseSheetShadowdBlock;
    use crate::helpers::random_access_file::RandomAccessFile;
    use std::cell::RefCell;
    use std::fs;
    use std::rc::Rc;
    use std::time::Instant;

    const BLOCKS: usize = 50_000;      // cantidad de escrituras
    const BLOCK_SIZE: usize = 4096;    // 4 KiB
    const FILE_SIZE: usize = BLOCKS * BLOCK_SIZE;

    #[test]
    fn read_test(){
        let raf = RandomAccessFile::new("stress/a/b/c/disk.bin")
            .expect("RAF no pudo crearse");
        let start = Instant::now();

        for block in (0..BLOCKS).rev() {
            let offset = block * BLOCK_SIZE;
            let expected = pattern(block);
            let read = raf.read_at(BLOCK_SIZE, offset as u64)
                .expect("Error de lectura");

            assert_eq!(read, expected, "Corrupción en bloque {}", block);
        }

        println!("Lectura completada en {:?}", start.elapsed());
    }
    #[test]
    fn write_test() {
        let _ = fs::remove_dir_all("stress");
        let raf = RandomAccessFile::new("stress/a/b/c/disk.bin")
            .expect("RAF no pudo crearse");
        let start = Instant::now();

        for block in 0..BLOCKS {
            let offset = block * BLOCK_SIZE;
            let data = pattern(block);
            raf.write_at(&data, offset as u64)
                .expect("Error de escritura");
        }

        raf.fsync().expect("sync falló");
    
        println!("Escritura completada en {:?}", start.elapsed());
    }
    fn pattern(block: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(BLOCK_SIZE);
        for i in 0..BLOCK_SIZE {
            v.push(((block ^ i) & 0xFF) as u8);
        }
        v
    }

    #[test]
    fn block_test(){
        let alloc = AllocatorBlock::new("Test");
        let alloc = Rc::new(RefCell::new(alloc));
        let mut block = BaseSheetShadowdBlock::new(
            0, 
            alloc, 
            0
        );
        let data_1 = "Un BaseSheetShadowdBlock es encencia el pilar de toda la infraestructura shadowd. ";
        let data_2 = "En si, un BSSB es solo un bloque que interactua con datos reales y con los datos de";
        let data_3 = "los demas bloques.";

        let mut data_1_v = Vec::new();
        data_1_v.extend_from_slice(data_1.as_bytes());
        let mut data_2_v = Vec::new();
        data_2_v.extend_from_slice(data_2.as_bytes());
        let mut data_3_v = Vec::new();
        data_3_v.extend_from_slice(data_3.as_bytes());

        let cur = Cursor::new(); // creamos un cursor apuntando a init

        assert_eq!(block.writed_bytes(), 0, "el bloque deberia de estar vacio");
        
        let result_1 = block.write_block(&cur, &mut data_1_v);
        assert!(result_1.is_some(), "algo fallo en escritura");
        let result_wb_1 = block.write_intern();
        assert!(result_wb_1.is_some(), "algo fallo en persistencia");
    }
}