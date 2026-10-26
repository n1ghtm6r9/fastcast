use crate::journal::JournalLines;
use std::fs::File;
use std::io::{self, Write};
use std::sync::Arc;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
pub struct JournalSink {
    lines: Arc<JournalLines>,
    file: Option<Arc<File>>,
}

impl JournalSink {
    pub fn new(lines: Arc<JournalLines>, file: Option<File>) -> Self {
        Self {
            lines,
            file: file.map(Arc::new),
        }
    }
}

impl Write for JournalSink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        for line in String::from_utf8_lossy(bytes).lines() {
            self.lines.push(line.to_owned());
        }
        if let Some(file) = &self.file {
            let _ = file.as_ref().write_all(bytes);
        }

        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for JournalSink {
    type Writer = JournalSink;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
