use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use async_trait::async_trait;
use futures::future::select_all;
use futures::select;
use pin_project_lite::pin_project;
use tokio_util::sync::CancellationToken;

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

pin_project! {
    pub struct ActorWrapper<ACTOR : Actor>
    {
        #[pin]
        pub inbox: ::tokio::sync::mpsc::UnboundedReceiver<ACTOR::MSG>,
        pub shutdown: bool,
        pub parent: Option<ActorParent>,
        pub actor: ACTOR,
    }
}

impl<ACTOR> Future for ActorWrapper<ACTOR> where ACTOR: Actor + Send
{
    type Output = Option<ACTOR::MSG>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut this = self.project();
        let mut inbox = this.inbox;
        let actor = this.actor;

        match inbox.poll_recv(cx) {
            Poll::Pending => {},
            Poll::Ready(r) => {
                match r {
                    None => {},
                    Some(r) => {
                        let mut on_msg = actor.on_msg(r);
                        Pin::new(&mut on_msg).poll(cx);
                    }
                }
            }
        };

        Poll::Pending
    }
}

impl<ACTOR> ActorWrapper<ACTOR>
    where ACTOR : Actor + Send,
{
    pub fn on_msg(msg: Option<ACTOR::MSG>) {

    }
}