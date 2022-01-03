use std::net::IpAddr;

#[derive(Debug)]
pub struct CicadaHeaders {
    pub user_agent: String,
    pub ip_address: IpAddr
}
