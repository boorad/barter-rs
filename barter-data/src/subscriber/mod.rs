use self::{
    mapper::{SubscriptionMapper, WebSocketSubMapper},
    validator::SubscriptionValidator,
};
use crate::{
    exchange::Connector,
    instrument::InstrumentData,
    subscription::{Map, Subscription, SubscriptionKind, SubscriptionMeta},
    Identifier,
};
use async_trait::async_trait;
use barter_integration::{
    error::SocketError,
    protocol::websocket::{WebSocket, WsMessage},
};
use futures::SinkExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use std::fmt::Debug;
use tracing::{debug, info};

/// [`SubscriptionMapper`] implementations defining how to map a
/// collection of Barter [`Subscription`]s into exchange specific [`SubscriptionMeta`].
pub mod mapper;

/// [`SubscriptionValidator`] implementations defining how to
/// validate actioned [`Subscription`]s were successful.
pub mod validator;

/// Defines how to connect to a socket
#[async_trait]
pub trait Connect {
    async fn connect<R>(request: R) -> Result<WebSocket, SocketError>
    where
        R: IntoClientRequest + Send + Unpin + Debug;
}

/// Defines how to subscribe to market data streams.
#[async_trait]
pub trait Subscriber {
    type SubMapper: SubscriptionMapper;

    async fn subscribe<Exchange, Instrument, Kind>(
        subscriptions: &[Subscription<Exchange, Instrument, Kind>],
    ) -> Result<Subscribed<Instrument::Key>, SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubscriptionKind + Send + Sync,
        Instrument: InstrumentData,
        Subscription<Exchange, Instrument, Kind>:
            Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

#[derive(Debug)]
pub struct Subscribed<InstrumentKey> {
    pub websocket: WebSocket,
    pub map: Map<InstrumentKey>,
    pub buffered_websocket_events: Vec<WsMessage>,
}

/// Standard [`Subscriber`] for [`WebSocket`]s suitable for most exchanges.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct WebSocketSubscriber;

#[async_trait]
impl Connect for WebSocketSubscriber {
    async fn connect<R>(request: R) -> Result<WebSocket, SocketError>
    where
        R: IntoClientRequest + Send + Unpin + Debug,
    {
        debug!(?request, "attempting to establish WebSocket connection");
        connect_async(request)
            .await
            .map(|(websocket, _)| websocket)
            .map_err(SocketError::WebSocket)
    }
}

#[async_trait]
impl Subscriber for WebSocketSubscriber {
    type SubMapper = WebSocketSubMapper;

    async fn subscribe<Exchange, Instrument, Kind>(
        subscriptions: &[Subscription<Exchange, Instrument, Kind>],
    ) -> Result<Subscribed<Instrument::Key>, SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubscriptionKind + Send + Sync,
        Instrument: InstrumentData,
        Subscription<Exchange, Instrument, Kind>:
            Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        // Define variables for logging ergonomics
        let exchange = Exchange::ID;
        let url = Exchange::url()?;
        debug!(%exchange, %url, ?subscriptions, "subscribing to WebSocket");

        // Connect to exchange
        let mut websocket = Self::connect(url).await?;
        debug!(%exchange, ?subscriptions, "connected to WebSocket");

        // Map &[Subscription<Exchange, Kind>] to SubscriptionMeta
        let SubscriptionMeta {
            instrument_map,
            ws_subscriptions,
        } = Self::SubMapper::map::<Exchange, Instrument, Kind>(subscriptions);

        // Send Subscriptions over WebSocket
        for subscription in ws_subscriptions {
            debug!(%exchange, payload = ?subscription, "sending exchange subscription");
            websocket.send(subscription).await?;
        }

        // Validate Subscription responses
        let (map, buffered_websocket_events) = Exchange::SubValidator::validate::<
            Exchange,
            Instrument::Key,
            Kind,
        >(instrument_map, &mut websocket)
        .await?;

        info!(%exchange, "subscribed to WebSocket");
        Ok(Subscribed {
            websocket,
            map,
            buffered_websocket_events,
        })
    }
}
