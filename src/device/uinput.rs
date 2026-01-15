use std::fs::OpenOptions;
use std::os::fd::OwnedFd;
use anyhow::Result;
use input_linux::UInputHandle;

pub type InputHandler = UInputHandle<OwnedFd>;

pub fn open_uinput() -> Result<InputHandler> {
    // Open the file located at /dev/uinput
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/uinput")?;

    // Convert the file into an owned file descriptor
    let fd = file.into();

    // Return the input handler to the file descriptor
    Ok(UInputHandle::new(fd))
}