use tokio::task::JoinHandle;

pub enum ProxyHandleMsg {

}

pub struct ProxyHandle {
    sender: ::tokio::sync::mpsc::Sender<ProxyHandleMsg>,
    jh: JoinHandle<()>,
}