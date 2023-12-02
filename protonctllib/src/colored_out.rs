use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, BufferWriter, WriteColor, Buffer, StandardStream};

pub struct StdOutStream {
    bufwtr: StandardStream,
}

pub struct StdOutBuff {
    buffer: Buffer,
    bufwtr: BufferWriter,
}

pub(crate) struct StdErrBuff {
    buffer: Buffer,
    bufwtr: BufferWriter,
}

impl StdOutBuff {
    pub fn new(choice: ColorChoice) -> Self {
        let bufwtr = BufferWriter::stdout(choice);
        let buffer = bufwtr.buffer();
        Self {
            buffer,
            bufwtr,
        }
    }

    pub fn set_color_spec(&mut self, spec: &ColorSpec) -> &mut Self {
        if let Err(e) = self.buffer.set_color(&spec) {
            eprintln!("Failed to set color: {e}");
        }
        self
    }

    pub fn write(&mut self, text: impl std::fmt::Display) -> &mut Self {
        if let Err(e) = write!(&mut self.buffer, "{}", text) {
            eprintln!("Failed to write to buffer: {e}");
        }
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        if let Err(e) = self.bufwtr.print(&self.buffer) {
            eprintln!("Failed to write buffer to output: {e}");
        }
        self.buffer.clear();
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        if let Err(e) = self.buffer.reset() {
            eprintln!("Failed to reset the buffer: {e}");
        }
        self
    }
}

impl StdErrBuff {
    pub fn new(choice: ColorChoice) -> Self {
        let bufwtr = BufferWriter::stderr(choice);
        let buffer = bufwtr.buffer();
        Self {
            buffer,
            bufwtr,
        }
    }

    pub fn set_color_spec(&mut self, spec: &ColorSpec) -> &mut Self {
        if let Err(e) = self.buffer.set_color(&spec) {
            eprintln!("Failed to set color: {e}");
        }
        self
    }

    pub fn write(&mut self, text: impl std::fmt::Display) -> &mut Self {
        if let Err(e) = write!(&mut self.buffer, "{}", text) {
            eprintln!("Failed to write to buffer: {e}");
        }
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        if let Err(e) = self.bufwtr.print(&self.buffer) {
            eprintln!("Failed to write buffer to output: {e}");
        }
        self.buffer.clear();
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        if let Err(_) = self.buffer.reset() {
            eprintln!("Failed to reset buffer");
        }
        self
    }
}

impl StdOutStream {
    pub fn new(choice: ColorChoice) -> Self {
        let bufwtr = StandardStream::stdout(choice);
        Self {
            bufwtr,
        }
    }

    pub fn set_color_spec(&mut self, spec: &ColorSpec) -> &mut Self {
        if let Err(e) = self.bufwtr.set_color(&spec) {
            eprintln!("Failed to set color: {e}");
        }
        self
    }

    pub fn write(&mut self, text: impl std::ops::Deref<Target = str>) -> &mut Self {
        if let Err(e) = self.bufwtr.write(text.as_bytes()) {
            eprintln!("Failed to write to buffer: {e}");
        }
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        if let Err(e) = self.bufwtr.flush() {
            eprintln!("Failed to write buffer to output: {e}");
        }
        self
    }
}

impl Drop for StdOutStream {
    fn drop(&mut self) {
        if let Err(_e) = self.bufwtr.reset() {
            eprintln!("Failed to reset standard stream before dropping");
        }
    }
}
