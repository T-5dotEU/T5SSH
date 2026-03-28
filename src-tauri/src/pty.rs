use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use tracing::debug;

pub struct PtyHandle {
    pub master_writer: Box<dyn Write + Send>,
    pub master_reader: Box<dyn Read + Send>,
    pub child: Box<dyn portable_pty::Child + Send>,
}

pub fn create_pty(
    command: CommandBuilder,
    rows: u16,
    cols: u16,
) -> Result<(PtyHandle, Box<dyn MasterPty + Send>), String> {
    let pty_system = native_pty_system();

    let pty_pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {}", e))?;

    let child = pty_pair
        .slave
        .spawn_command(command)
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let reader = pty_pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone PTY reader: {}", e))?;

    let writer = Box::new(DebugMouseWriter::new(
        pty_pair
            .master
            .take_writer()
            .map_err(|e| format!("Failed to take PTY writer: {}", e))?
    ));

    Ok((
        PtyHandle {
            master_writer: writer,
            master_reader: reader,
            child,
        },
        pty_pair.master,
    ))
}

pub fn resize_pty(master: &dyn MasterPty, rows: u16, cols: u16) -> Result<(), String> {
// Wrapper for PTY writer to log mouse escape sequences
struct DebugMouseWriter<W: Write> {
    inner: W,
}

impl<W: Write> DebugMouseWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<W: Write> Write for DebugMouseWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() >= 3 && buf[0] == 0x1b && buf[1] == b'[' && (buf[2] == b'<' || (buf[2] >= b'0' && buf[2] <= b'9')) {
            debug!(
                hex = %buf.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "),
                raw = ?buf,
                marker = "[T5SSH-MOUSE-PTY-WRITE]",
                "Mouse escape sequence written to PTY"
            );
        }
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
    master
        .resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to resize PTY: {}", e))
}
