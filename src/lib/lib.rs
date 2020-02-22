use std::net::{IpAddr, Ipv4Addr};

pub const PROXY_SERVER_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10000);
pub const VC_PIAZZA_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10001);
pub const PI_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10002);
pub const AMIT_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10003);
// pub const PROXY_SERVER_ADDR: &SocketAddr = SocketAddr::from(([127, 0, 0, 1], 10000));
// pub const VC_PIAZZA_ADDR: &SocketAddr = SocketAddr::from(([127, 0, 0, 1], 10001));
// pub const PI_ADDR: &SocketAddr = SocketAddr::from(([127, 0, 0, 1], 10000));
// pub const AMIT_ADDR: &SocketAddr = SocketAddr::from(([127, 0, 0, 1], 10000));
pub mod ifc;
pub mod ifc_grpc;
