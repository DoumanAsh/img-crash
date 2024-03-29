use mime_guess::Mime;
use memmap::{Mmap, MmapMut};
use std::fs::{File, OpenOptions};
use std::io;

pub struct Image {
    inner: Mmap,
    file: File,
    pub len: u64,
    mime: Mime
}

impl Image {
    pub fn open(path: &str) -> io::Result<Image> {
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        if mime != mime::IMAGE_PNG {
            return Err(io::Error::new(io::ErrorKind::Other, "Unsupported file type. Available: png."))
        }

        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let len = file.metadata()?.len();
        let inner = unsafe { Mmap::map(&file)? };

        Ok(Image {
            file,
            inner,
            len,
            mime
        })
    }

    pub fn update(self, new: &[u8]) -> io::Result<()> {
        drop(self.inner);
        self.file.set_len(new.len() as u64)?;
        let mut map = unsafe { MmapMut::map_mut(&self.file)? };
        map.copy_from_slice(new);
        map.flush()
    }

    #[inline]
    pub fn slice(&self) -> &[u8] {
        &self.inner[..]
    }

    #[inline]
    pub fn is_png(&self) -> bool {
        self.mime == mime::IMAGE_PNG
    }
}
