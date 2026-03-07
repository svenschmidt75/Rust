use crate::emitter::Emitter;

pub(crate) struct StringEmitter {
    pub(crate) buffer: Vec<String>,
}

impl StringEmitter {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl Emitter for StringEmitter {
    fn emit(&mut self, text: &str) {
        self.buffer.push(text.to_string());
    }

    fn finish(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
