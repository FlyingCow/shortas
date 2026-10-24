//! Message queue services using RabbitMQ.

pub mod outbox_processor;
pub mod rabbitmq_consumer;

pub use outbox_processor::*;
pub use rabbitmq_consumer::*;
