use anyhow::{Context, Result};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::Mutex;

pub struct CommunicationBus<T> {
    /// The producer channel (sender)
    sender: Mutex<Sender<T>>,

    /// The consumer channel (receiver)
    receiver: Mutex<Receiver<T>>,
}

impl<T> CommunicationBus<T> {
    pub fn new() -> Self {
        // Create a channel with multiple producers and consumers allowed
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: Mutex::new(sender),
            receiver: Mutex::new(receiver),
        }
    }

    pub fn send_data(&self, data: T) -> Result<()> {
        // Attempt to lock the sender channel
        let sender = self
            .sender
            .lock()
            .map_err(|err| anyhow::anyhow!("Failed to acquire lock on sender: {}", err))
            .with_context(|| "Sender mutex has been poisoned")?;

        // Attempt to send the data along the sender channel
        sender
            .send(data)
            .map_err(|err| anyhow::anyhow!("Failed to send data: {}", err))
            .with_context(|| "Receiver channel has disconnected")?;

        // Return successful sending of the data
        Ok(())
    }

    pub fn receive_data(&self) -> Result<T> {
        // Attempt to lock the receiver channel
        let receiver = self
            .receiver
            .lock()
            .map_err(|err| anyhow::anyhow!("Failed to acquire lock on receiver: {}", err))
            .with_context(|| "Receiver mutex has been poisoned")?;

        // Attempt to receive data from the channel (this is blocking)
        let data = receiver.recv()
            .map_err(|err| anyhow::anyhow!("Failed to receive data: {}", err))
            .with_context(|| "Sender channel has disconnected")?;

        // Return the received data
        Ok(data)
    }

    pub fn try_receive_data(&self) -> Result<Option<T>> {
        // Attempt to lock the receiver channel
        let receiver = self.receiver.lock()
            .map_err(|err| anyhow::anyhow!("Failed to acquire lock on receiver: {}", err))
            .with_context(|| "Receiver mutex has been poisoned")?;

        // Attempt to receive data from the channel (this is non-blocking)
        match receiver.try_recv() {
            Ok(data) => Ok(Some(data)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => anyhow::bail!("Receiver channel has disconnected"),
        }
    }
}
