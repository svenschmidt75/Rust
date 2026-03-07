pub trait Emitter {
    fn emit(&mut self, text: &str);
    fn finish(&mut self) -> std::io::Result<()>;
}
