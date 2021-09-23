#[derive(Clone)]
pub struct Host {
    protocol: String,
    host_addr: String,
    port: String,
}

impl Host {
    pub fn new(protocol: String, host_addr: String, port: String) -> Host {
        Host {
            protocol,
            host_addr,
            port,
        }
    }

    pub fn endpoint(&self) -> String {
        format!("{}//{}:{}", self.protocol, self.host_addr, self.port)
    }
    pub fn tendermint_endpoint(&self) -> String {
        format!(
            "{}//tendermint.{}:{}",
            self.protocol, self.host_addr, self.port
        )
    }
    pub fn grpc_endpoint(&self) -> String {
        format!("{}//grpc.{}:{}", self.protocol, self.host_addr, self.port)
    }
    pub fn faucet_endpoint(&self) -> String {
        format!("{}//faucet.{}:{}", self.protocol, self.host_addr, self.port)
    }
}
