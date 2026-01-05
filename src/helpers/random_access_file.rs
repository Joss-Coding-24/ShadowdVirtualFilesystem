use std::{
    fs::{
        File, 
        OpenOptions
    },
    io::{
        self, 
        Read
    },
    os::unix::fs::FileExt,
    sync::Arc,
};

pub struct RandomAccessFile {
    file: Arc<File>,
}

impl RandomAccessFile {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: Arc::new(
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)?
            )
        })
    }

    pub fn write_at(&self, data: &[u8], offset: u64) -> io::Result<()> {
        let mut written = 0;

        while written < data.len() {
            let n = self.file.write_at(
                &data[written..],
                offset + written as u64,
            )?;
            written += n;
        }

        Ok(())
    }

    pub fn read_at(&self, count: usize, offset: u64) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; count];
        let mut read = 0;
        if self.size()?<count as u64 {return Ok(buf)}//se que no deberia de hacer esto, pero es un por ai acaso

        while read < count {
            let n = self.file.read_at(
                &mut buf[read..],
                offset + read as u64,
            )?;
            if n == 0 {
                break;
            }
            read += n;
        }

        buf.truncate(read);
        Ok(buf)
    }

    pub fn size(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.file.sync_all()
    }
}