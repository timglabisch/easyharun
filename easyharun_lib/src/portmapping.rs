use std::collections::HashMap;
use anyhow::anyhow;

pub struct PortMapping {
    pub listen_addr: String,
    pub server_addr: String,
}

pub struct PortMappingParser;

impl PortMappingParser {
    // [BLOCK],[BLOCK]
    pub fn parse(str : &str) -> Result<Vec<PortMapping>, ::anyhow::Error> {

        let mut buf = vec![];
        for s in str.split(",") {
            buf.push(Self::parse_single_block(s)?);
        }

        Ok(buf)
    }

    // 8080 -> listen_addr = 8080, server_addr = 8080
    // 8080:80 -> listen_addr = 80, server_addr = X:8080
    fn parse_single_block(str : &str) -> Result<PortMapping, ::anyhow::Error> {
        let ports = str.split(":").map(|v|v.trim().to_string()).collect::<Vec<String>>();

        return Ok(match ports.len() {
            1 => PortMapping {
                server_addr: format!("0.0.0.0:{}", ports[0]),
                listen_addr: format!("0.0.0.0:{}", ports[0])
            },
            2 => PortMapping {
                server_addr: format!("0.0.0.0:{}", ports[0]),
                listen_addr: format!("0.0.0.0:{}", ports[1])
            },
            _ => return Err(anyhow!("invalid port mapping {}", str))
        });
    }
}