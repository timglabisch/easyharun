use std::future::{Future, poll_fn};
use std::marker::PhantomData;
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

    async fn on_msg(&mut self) {
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

pub enum ActorMsg<T> {
    MSG(T),
    KILL,
    PING(ActorMsgPingRequest),
}

    pub struct ActorWrapper<ACTOR>
    where ACTOR : Actor,
    {
        pub inbox: ::tokio::sync::mpsc::UnboundedReceiver<ActorMsg<ACTOR::MSG>>,
        pub shutdown: bool,
        pub parent: Option<ActorParent>,
        pub actor: ACTOR,
        pub future_drive_onmsg: Option<Pin<Box<dyn Future<Output = ()> + Send>>>,
    }


impl<'a, ACTOR> Future for ActorWrapper<ACTOR>
    where ACTOR: Actor + Send + Unpin
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        //let mut this = self.project();
        // let mut inbox = this.inbox;
        // let actor = this.actor;

        // let this = self.as_mut();

        let this = Pin::get_mut(self);

        /*
        match self.future_drive_onmsg.take() {

        }
         */

        match this.inbox.poll_recv(cx) {
            Poll::Pending => {},
            Poll::Ready(r) => {
                let msg = match r {
                    None => return Poll::Ready(()),
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

                        let on_msg = this.actor.on_msg();
                        this.future_drive_onmsg.insert(on_msg);

                        // let poll = Pin::new(&mut on_msg).poll(cx);
                    }
                }
            }
        };

        Poll::Pending
    }
}