use crate::proxy::world::{ProxyWorldEntry, ProxyWorlds};

pub struct ProxyBrain;

#[derive(Debug, Clone)]
pub enum ProxyBrainAction {
    Add(ProxyBrainActionAdd),
    RemoveAsk(ProxyBrainActionRemove),
}

#[derive(Debug, Clone)]
pub struct ProxyBrainActionAdd {
    pub listen_addr: String,
    pub server_addr: String,
}

#[derive(Debug, Clone)]
pub struct ProxyBrainActionRemove {
    pub listen_addr: String,
    pub server_addr: String,
}

impl ProxyBrain {
    pub fn think(worlds : &ProxyWorlds) -> Vec<ProxyBrainAction> {

        let mut buf = vec![];

        buf.extend(Self::think_about_adding_proxies(worlds));
        buf.extend(Self::think_about_removing_proxies(worlds));

        return buf;
    }

    fn think_about_removing_proxies(worlds : &ProxyWorlds) -> Vec<ProxyBrainAction> {
        let mut buf = vec![];

        for (_, proxy_current) in worlds.current.proxies.iter() {
            for proxy_current_addr in proxy_current.server_addrs.iter() {

                let existing_proxy = match worlds.expected.proxies.get(&proxy_current.listen_addr) {
                    None => {
                        // entire proxy does not exist
                        buf.push(ProxyBrainAction::RemoveAsk(ProxyBrainActionRemove {
                            listen_addr: proxy_current.listen_addr.to_string(),
                            server_addr: proxy_current_addr.to_string(),
                        }));
                        continue;
                    }
                    Some(s) => s,
                };

                if existing_proxy.server_addrs.contains(proxy_current_addr) {
                    continue
                }

                // server addr is not registered in proxy
                buf.push(ProxyBrainAction::RemoveAsk(ProxyBrainActionRemove {
                    listen_addr: proxy_current.listen_addr.to_string(),
                    server_addr: proxy_current_addr.to_string(),
                }));
            }
        }

        buf
    }

    fn think_about_adding_proxies(worlds : &ProxyWorlds) -> Vec<ProxyBrainAction> {
        let mut buf = vec![];

        for (_, proxy_expected) in worlds.expected.proxies.iter() {
            for proxy_expected_addr in proxy_expected.server_addrs.iter() {
                let existing_proxy = match worlds.current.proxies.get(&proxy_expected.listen_addr) {
                    None => {
                        // entire proxy does not exist
                        buf.push(ProxyBrainAction::Add(ProxyBrainActionAdd {
                            listen_addr: proxy_expected.listen_addr.to_string(),
                            server_addr: proxy_expected_addr.to_string(),
                        }));
                        continue;
                    }
                    Some(s) => s,
                };

                if existing_proxy.server_addrs.contains(proxy_expected_addr) {
                    continue
                }

                // server addr is not registered in proxy
                buf.push(ProxyBrainAction::Add(ProxyBrainActionAdd {
                    listen_addr: proxy_expected.listen_addr.to_string(),
                    server_addr: proxy_expected_addr.to_string(),
                }));
            }

        }

        buf
    }
}