use std::sync::mpsc::{self, Receiver, Sender};

pub struct AppChannel<T> {
    /// The producer channel (sender)
    producer: Sender<T>,

    /// The consumer channel (receiver)
    consumer: Option<Receiver<T>>,
}

impl<T> AppChannel<T> {
    pub fn new() -> Self {
        // Create an unbounded channel
        let (producer, consumer) = mpsc::channel();
        Self {
            producer,
            consumer: Some(consumer),
        }
    }

    pub fn get_producer(&self) -> Sender<T> {
        self.producer.clone()
    }

    pub fn get_consumer(&mut self) -> Option<Receiver<T>> {
        self.consumer.take()
    }
}
