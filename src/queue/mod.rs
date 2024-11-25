use std::error::Error;

mod rabbitmq_queue;
mod rust_queue;

pub use rabbitmq_queue::RabbitmqQueue;
pub use rust_queue::RustQueue;

pub trait Acker {
    fn queue_ack(&self) -> Result<(), Box<dyn Error>> {
        println!("ack");
        Ok(())
    }
    fn queue_nack(&self) -> Result<(), Box<dyn Error>> {
        println!("nack");
        Ok(())
    }
    fn queue_reject(&self) -> Result<(), Box<dyn Error>> {
        println!("reject");
        Ok(())
    }
}

pub struct Message {
    pub data: Vec<u8>,
    pub acker: Box<dyn Acker>,
}
