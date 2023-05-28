use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use async_trait::async_trait;
use futures::future::select_all;
use futures::select;
use pin_project_lite::pin_project;
use tokio_util::sync::CancellationToken;
use tracing::trace;

#[async_trait]
pub trait Actor
{
    type MSG : Sized + Send;

    async fn on_msg(&mut self, msg : Self::MSG) {
        ()
    }
}

pub struct ActorParent {
    cancellation_token: CancellationToken,
}

pub struct ActorMsgPingResponse {
    id: usize,
}

pub struct ActorMsgPingRequest {
    oneshot: ::tokio::sync::oneshot::Sender<ActorMsgPingResponse>,
    id: usize,
}

enum ActorMsg<T> {
    MSG(T),
    KILL,
    PING(ActorMsgPingRequest),
}

pin_project! {
    pub struct ActorWrapper<ACTOR : Actor>
    {
        #[pin]
        pub inbox: ::tokio::sync::mpsc::UnboundedReceiver<ActorMsg<ACTOR::MSG>>,
        pub shutdown: bool,
        pub parent: Option<ActorParent>,
        pub actor: ACTOR,
    }
}

impl<ACTOR> Future for ActorWrapper<ACTOR> where ACTOR: Actor + Send
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut this = self.project();
        let mut inbox = this.inbox;
        let actor = this.actor;

        match inbox.poll_recv(cx) {
            Poll::Pending => {},
            Poll::Ready(r) => {
                let msg = match r {
                    None => return Poll::Ready(None),
                    Some(r) => r
                };

                match msg {
                    ActorMsg::KILL => {
                        return Poll::Ready(())
                    },
                    ActorMsg::PING(ping_response) => {
                        match ping_response.oneshot.send(ActorMsgPingResponse {
                            id: ping_response.id
                        }) {
                            Ok(_) => {},
                            Err(e) => {
                                trace!("could not send ping response.")
                            }
                        }
                    },
                    ActorMsg::MSG(generic_msg) => {
                        let mut on_msg = actor.on_msg(generic_msg);
                        let poll = Pin::new(&mut on_msg).poll(cx);

                        
                    }
                }
            }
        };

        Poll::Pending
    }
}