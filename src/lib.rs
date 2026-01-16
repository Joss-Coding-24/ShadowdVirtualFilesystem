use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static GLOBAL_RUNTIME: Lazy<Arc<Runtime>> = Lazy::new(
    || {
        Arc::new(
            Runtime::new().expect("Failed to create Tokio runtime")
        )
    }
);

pub mod block;
pub mod algoritm;
pub mod helpers;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::algoritm::cursors::Cursor;
    use crate::block::{
        shadowd_block::Block,
        allocator_block::AllocatorBlock,
        base_sheet_shadowd_block::BaseSheetShadowdBlock
    };
    use crate::helpers::random_access_file::RandomAccessFile;
    use std::{
        cell::RefCell,
        fs,
        rc::Rc,
        time::Instant
    };

    const BLOCKS: usize = 50_000;      // cantidad de escrituras
    const BLOCK_SIZE: usize = 4096;    // 4 KiB

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
            alloc.clone(), 
            0,
            1
        );
        let data_1 = "Un BaseSheetShadowdBlock es encencia el pilar de toda la infraestructura shadowd. ";
        let mut data_1_v = Vec::new();
        data_1_v.extend_from_slice(data_1.as_bytes());
        let data_1_l = data_1_v.len().clone();
        let cur = Cursor::new(); // creamos un cursor apuntando a init

        assert_eq!(block.writed_bytes(), 0, "el bloque deberia de estar vacio");
        
        let result_1 = block.write_block(&cur, &mut data_1_v.clone());
        assert!(result_1.is_some(), "algo fallo en escritura");
        let u_result_1 = result_1.unwrap();
        assert_eq!(u_result_1.remaining, 0, "No deberian de sobrar bytes aqui");
        assert_eq!(u_result_1.remaining+u_result_1.written, data_1_l, "No se escribieron todos los bytes");
        let result_wb_1 = block.write_intern();
        assert!(result_wb_1.is_some(), "algo fallo en persistencia");

        let mut block2 = BaseSheetShadowdBlock::new(
            0, 
            alloc.clone(), 
            0, 
            1
        );
        let result_2 = block2.read_to(&cur, data_1_l);
        assert!(result_2.is_some(), "algo salio mal leyendo");
        let val = result_2.unwrap();
        assert_eq!(val, data_1_v, "La ofuscacion tiene errores")
    }
}