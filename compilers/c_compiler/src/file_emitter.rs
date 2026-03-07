use crate::emitter::Emitter;
use std::io::BufWriter;
use std::io::Write;

pub(crate) struct FileEmitter {
    writer: BufWriter<std::fs::File>,
}

impl FileEmitter {
    pub fn new(path: &std::path::Path) -> std::io::Result<Self> {
        let writer = BufWriter::new(std::fs::File::create(path)?);
        Ok(Self { writer })
    }
}

impl Emitter for FileEmitter {
    fn emit(&mut self, text: &str) {
        writeln!(self.writer, "{}", text).expect("Failed to write to buffer");
    }

    fn finish(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl Drop for FileEmitter {
    fn drop(&mut self) {
        self.writer.flush().expect("Failed to flush file");
    }
}
