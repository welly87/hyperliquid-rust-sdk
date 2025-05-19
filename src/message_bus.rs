use std::marker::PhantomData;
use std::time::Duration;

use async_nats::{
    client::PublishError, client::RequestError, client::SubscribeError, connect, Client, Subscriber,
};
use futures::StreamExt;
use thiserror::Error;
use uuid::Uuid;

use crate::messages::{Message, MessageError, MessageHeader};

#[derive(Error, Debug)]
pub enum BusError {
    #[error("publish error: {0}")]
    Publish(#[from] PublishError),
    #[error("request error: {0}")]
    Request(#[from] RequestError),
    #[error("subscribe error: {0}")]
    Subscribe(#[from] SubscribeError),
    #[error("message error: {0}")]
    Message(#[from] MessageError),
    #[error("timeout")]
    Timeout,
}

/// Simple message bus over NATS.
pub struct MessageBus {
    client: Client,
}

impl MessageBus {
    /// Connect to the NATS server at the given URL.
    pub async fn connect(url: &str) -> Result<Self, async_nats::Error> {
        let client = connect(url).await?;
        Ok(Self { client })
    }

    /// Send a message without waiting for a response.
    pub async fn send<M: Message>(&self, subject: &str, msg: &M) -> Result<(), BusError> {
        let bytes = serialize_with_correlation(msg, None)?;
        self.client
            .publish(subject.to_string(), bytes.into())
            .await?;
        Ok(())
    }

    /// Publish a message to all subscribers.
    pub async fn publish<M: Message>(&self, subject: &str, msg: &M) -> Result<(), BusError> {
        self.send(subject, msg).await
    }

    /// Send a request and wait for a typed response.
    pub async fn request<Req: Message, Resp: Message>(
        &self,
        subject: &str,
        req: &Req,
        timeout_dur: Duration,
    ) -> Result<Resp, BusError> {
        let correlation_id = Uuid::new_v4().into_bytes();
        let bytes = serialize_with_correlation(req, Some(correlation_id))?;
        let msg = tokio::time::timeout(
            timeout_dur,
            self.client.request(subject.to_string(), bytes.into()),
        )
        .await
        .map_err(|_| BusError::Timeout)??;

        let (header, resp) = deserialize_with_header::<Resp>(msg.payload.as_ref())?;
        if header.correlation_id != Some(correlation_id) {
            return Err(MessageError::InvalidFormat("correlation id mismatch".into()).into());
        }
        Ok(resp)
    }

    /// Subscribe to a subject and receive typed messages.
    pub async fn subscribe<M: Message>(
        &self,
        subject: &str,
    ) -> Result<BusSubscription<M>, BusError> {
        let sub = self.client.subscribe(subject.to_string()).await?;
        Ok(BusSubscription {
            inner: sub,
            _phantom: PhantomData,
        })
    }
}

/// Typed subscription wrapper.
pub struct BusSubscription<M: Message> {
    inner: Subscriber,
    _phantom: PhantomData<M>,
}

impl<M: Message> BusSubscription<M> {
    /// Receive the next message from the subscription.
    pub async fn next(&mut self) -> Option<Result<M, BusError>> {
        match self.inner.next().await {
            Some(msg) => Some(
                deserialize_with_header::<M>(msg.payload.as_ref())
                    .map(|(_, m)| m)
                    .map_err(BusError::from),
            ),
            None => None,
        }
    }
}

fn serialize_with_correlation<M: Message>(
    msg: &M,
    cid: Option<[u8; 16]>,
) -> Result<Vec<u8>, MessageError> {
    let mut header = MessageHeader::new(M::message_type());
    if let Some(cid) = cid {
        header = header.with_correlation_id(cid);
    }
    let header_bytes = rmp_serde::to_vec_named(&header)?;
    let body_bytes = rmp_serde::to_vec_named(msg)?;
    let mut out = Vec::with_capacity(header_bytes.len() + body_bytes.len() + 4);
    out.extend_from_slice(&(header_bytes.len() as u32).to_be_bytes());
    out.extend(header_bytes);
    out.extend(body_bytes);
    Ok(out)
}

fn deserialize_with_header<M: Message>(data: &[u8]) -> Result<(MessageHeader, M), MessageError> {
    if data.len() < 4 {
        return Err(MessageError::InvalidFormat("message too short".into()));
    }
    let header_len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + header_len {
        return Err(MessageError::InvalidFormat("invalid header length".into()));
    }
    let header: MessageHeader = rmp_serde::from_slice(&data[4..4 + header_len])?;
    if header.msg_type != M::message_type() {
        return Err(MessageError::MismatchedType {
            expected: M::message_type(),
            actual: header.msg_type,
        });
    }
    if header.is_expired() {
        return Err(MessageError::Expired);
    }
    let msg = rmp_serde::from_slice(&data[4 + header_len..])?;
    Ok((header, msg))
}
