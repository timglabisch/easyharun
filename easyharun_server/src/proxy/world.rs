use std::collections::{HashMap, HashSet};

pub struct ProxyWorlds {
    pub current: ProxyWorld,
    pub expected: ProxyWorld,
}

pub struct ProxyWorld {
    pub proxies: HashMap<String, ProxyWorldEntry>,
}

pub struct ProxyWorldEntry {
    pub listen_addr: String,
    pub server_addrs: HashSet<String>,
}

impl ProxyWorlds {
    pub fn build_diff() {}
}

impl ProxyWorld {

}