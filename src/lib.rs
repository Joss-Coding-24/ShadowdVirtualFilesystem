pub mod block;
pub mod algoritm;
pub mod helpers;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::helpers::random_access_file::RandomAccessFile;
    use std::fs;
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
    fn gen_test(){
        
    }
}