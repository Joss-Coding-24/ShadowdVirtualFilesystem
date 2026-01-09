use std::{
    fs::{
        self, 
        File, 
        OpenOptions
    }, 
    io::{
        self
    },
    os::unix::fs::FileExt, 
    path::Path, 
    sync::Arc
};

pub struct RandomAccessFile {
    file: Arc<File>,
}

impl RandomAccessFile {
    pub fn new(path_: impl AsRef<Path>) -> io::Result<Self> {
        let path = Path::new(path_.as_ref().as_os_str());
        ensure_parent_dir(path)?;
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
        let size = self.size()?;
        if offset + count as u64 > size {
            return Ok(Vec::new())
        }

        let mut buf = vec![0u8; count];
        let mut read = 0;

        while read < count {
            let n = self.file.read_at(
                &mut buf[read..],
                offset + read as u64,
            )?;
            if n == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "lectura incompleta",
                ));
            }
            read += n;
        }

        Ok(buf)
    }

    pub fn size(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.file.sync_all()
    }
}

fn ensure_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}