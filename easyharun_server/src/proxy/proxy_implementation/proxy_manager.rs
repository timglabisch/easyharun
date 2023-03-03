use std::collections::HashMap;

pub struct ManagedProxy {
    listen_addr: String,
    server_addrs: Vec<String>,
}

pub struct ProxyManager {
    proxies: HashMap<String, ManagedProxy>
}



