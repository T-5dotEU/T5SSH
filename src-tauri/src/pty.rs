use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};

pub struct PtyHandle {
    pub master_writer: Box<dyn Write + Send>,
    pub master_reader: Box<dyn Read + Send>,
    pub child: Box<dyn portable_pty::Child + Send>,
}

pub fn create_pty(
    cmd: &str,
    args: &[String],
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

    let mut command = CommandBuilder::new(cmd);
    for arg in args {
        command.arg(arg);
    }

    let child = pty_pair
        .slave
        .spawn_command(command)
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let reader = pty_pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone PTY reader: {}", e))?;

    let writer = pty_pair
        .master
        .take_writer()
        .map_err(|e| format!("Failed to take PTY writer: {}", e))?;

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
    master
        .resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to resize PTY: {}", e))
}
