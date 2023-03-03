use std::collections::HashMap;

pub struct ProxyWorlds {
    pub current: ProxyWorld,
    pub expected: ProxyWorld,
}

pub struct ProxyWorld {
    pub proxies: HashMap<String, ProxyWorldEntry>,
}

pub struct ProxyWorldEntry {
    pub listen_addr: String,
    pub server_addrs: Vec<String>,
}

impl ProxyWorlds {
    pub fn build_diff() {}
}

impl ProxyWorld {

}