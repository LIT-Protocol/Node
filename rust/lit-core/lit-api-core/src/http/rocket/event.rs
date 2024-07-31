use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::sync::mpsc::Sender;

use crate::error::Result;

pub(crate) type EventAction =
    Arc<dyn Send + Sync + Fn(&HashMap<EventDataKey, Vec<u8>>) -> BoxFuture<'static, Result<()>>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Event {
    CertChange,
    CertVerify,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum EventDataKey {
    CommonName,
    ValidationId,
    ValidationContent,
    CertificateId,
    CertificateCrt,
    CaBundleCrt,
}

#[derive(Clone)]
pub struct EventManager {
    actions: HashMap<Event, Vec<EventAction>>,
    grpc_cert_channel: Option<Sender<bool>>,
}

impl EventManager {
    pub fn new(grpc_cert_channel: Option<Sender<bool>>) -> Self {
        Self { actions: HashMap::new(), grpc_cert_channel }
    }

    pub fn on_event(
        &mut self, event: Event,
        action: impl Send
            + Sync
            + Fn(&HashMap<EventDataKey, Vec<u8>>) -> BoxFuture<'static, Result<()>>
            + 'static,
    ) -> &mut Self {
        let mut actions = if self.actions.contains_key(&event) {
            self.actions.remove(&event).unwrap()
        } else {
            Vec::new()
        };

        actions.push(Arc::new(action));

        self.actions.insert(event, actions);

        self
    }

    pub async fn trigger_event(
        &self, event: Event, data: HashMap<EventDataKey, Vec<u8>>,
    ) -> Result<()> {
        if self.actions.contains_key(&event) {
            for action in self.actions.get(&event).unwrap() {
                action(&data).await?;
            }
        }

        if event == Event::CertChange {
            match self.grpc_cert_channel {
                Some(ref tx) => match tx.send(true).await {
                    Ok(_) => {
                        info!("Sent cert change event to grpc controller");
                    }
                    Err(e) => {
                        error!("Grpc server cert change channel error: {:?}", e);
                    }
                },
                None => {
                    warn!("Grpc server cert change channel not set");
                }
            }
        }

        Ok(())
    }
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager::new(None)
    }
}
