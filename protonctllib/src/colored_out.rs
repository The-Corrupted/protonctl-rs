use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, BufferWriter, WriteColor, Buffer};

pub struct StdOut {
    buffer: Buffer,
    bufwtr: BufferWriter,
    colorspec: ColorSpec,
}

pub struct StdErr {
    buffer: Buffer,
    bufwtr: BufferWriter,
}

impl StdOut {
    pub fn new(choice: ColorChoice) -> Self {
        let bufwtr = BufferWriter::stdout(choice);
        let buffer = bufwtr.buffer();
        let colorspec = ColorSpec::new();
        Self {
            buffer,
            bufwtr,
            colorspec,
        }
    }

    pub fn get_color_spec(&self) -> ColorSpec {
        return self.colorspec.clone();
    }

    pub fn set_color_spec(&mut self, spec: &ColorSpec) -> &mut Self {
        self.colorspec = spec.clone();
        self
    }

    pub fn write(&mut self, text: impl std::fmt::Display + ToString) -> anyhow::Result<&mut Self> {
        let _ = self.buffer.set_color(&self.colorspec);
        if let Err(e) = write!(self.buffer, "{}", text) {
            return Err(anyhow::anyhow!("Failed to write to buffer: {}", e));
        }
        Ok(self)
    }

    pub fn flush(&mut self) -> anyhow::Result<&mut Self> {
        if let Err(e) = self.bufwtr.print(&self.buffer) {
            return Err(anyhow::anyhow!("Failed to print to screen: {}", e));
        }
        Ok(self)
    }
}
