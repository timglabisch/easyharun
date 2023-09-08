use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone, Debug)]
pub struct DebugWrite {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DebugWrite {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_data(&mut self) -> Vec<u8> {
        self.data.lock().expect("could not get lock").clone()
    }

    pub fn dump(&mut self) {
        println!("DUMP: {}", String::from_utf8_lossy(&self.get_data()));
    }
}

impl Write for DebugWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut lock = self.data.lock().expect("could not get lock");
        lock.extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl MakeWriter<'_> for DebugWrite {
    type Writer = DebugWrite;

    fn make_writer(&self) -> Self::Writer {
        self.clone()
    }
}