use beijing::UserIdScopes;
use civilization::utils::make_internal;
use futures::Stream;
use messages_proto::{messages_v1_server::*, Msg, MsgStream};
use std::{collections::BTreeMap, pin::Pin, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use tonic::{Request, Response, Status};

struct Senders(BTreeMap<i64, mpsc::Sender<MsgStream>>);

impl Senders {
    fn new() -> Self {
        Senders(BTreeMap::new())
    }

    async fn notify(&self, msg: MsgStream) {
        for (user_id, tx) in &self.0 {
            if msg.receiver_id == *user_id {
                if let Err(err) = tx.send(msg.clone()).await {
                    tracing::error!("{}", err);
                }
            }
        }
    }
}

struct MessagesApiV1 {
    senders: Arc<RwLock<Senders>>,
}

impl MessagesApiV1 {
    fn new() -> Self {
        Self {
            senders: Arc::new(RwLock::new(Senders::new())),
        }
    }
}

#[tonic::async_trait]
impl MessagesV1 for MessagesApiV1 {
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<MsgStream, Status>> + Send + Sync + 'static>>;

    async fn subscribe(&self, req: Request<()>) -> Result<Response<Self::SubscribeStream>, Status> {
        let UserIdScopes { user_id, .. } = req
            .extensions()
            .get::<UserIdScopes>()
            .ok_or(make_internal("Can't get 'UserIdScopes' extension"))?;

        let (stream_tx, stream_rx) = mpsc::channel(1);

        let (tx, rx) = mpsc::channel(1);
        self.senders.write().await.0.insert(*user_id, tx);

        let senders_cloned = self.senders.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        tracing::debug!("Stream tx sending error, Remote user id {}", user_id);
                        senders_cloned.write().await.0.remove(user_id);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))
    }

    async fn send_msg(&self, req: Request<Msg>) -> Result<Response<()>, Status> {
        let UserIdScopes { user_id, .. } = req
            .extensions()
            .get::<UserIdScopes>()
            .ok_or(make_internal("Can't get 'UserIdScopes' extension"))?;

        
    }
}
