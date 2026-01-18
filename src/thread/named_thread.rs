use anyhow::{Context, Result};
use std::sync::Mutex;
use std::thread;

pub type JoinHandle = thread::JoinHandle<()>;

pub struct NamedThread {
    name: &'static str,
    handle: Mutex<Option<JoinHandle>>,
}

impl NamedThread {
    pub fn new(name: &'static str) -> Result<Self> {
        // Return the named thread object
        Ok(Self {
            name,
            handle: Mutex::new(None),
        })
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn spawn<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() -> () + Send + 'static,
        (): Send + 'static,
    {
        // Spawn a new named thread using the builder
        let join_handle = thread::Builder::new()
            .name(self.name.to_owned())
            .spawn(task)?;

        // Attempt to obtain the lock on the thread join handle
        let mut guard = self.handle.lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock thread handle: {}", e))
            .with_context(|| format!("Failed to store join handle for the {} thread", self.name))?;

        // Store the join handle now that the lock have been obtained
        *guard = Some(join_handle);

        // Return spawn success
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        // Attempt to obtain the lock on the thread join handle
        let mut guard = self.handle.lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock thread handle: {}", e))
            .with_context(|| format!("Failed to stop {} thread", self.name))?;

        // Take the join handle and replace it with None as the thread should be stopped
        if let Some(handle) = guard.take() {
            // Attempt to stop join the thread into the parent thread
            if let Err(e) = handle.join() {
                anyhow::bail!("Failed to join thread as thread panicked: {:?}", e);
            }
        }

        // Return join success
        Ok(())
    }
}
