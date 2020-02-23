use std::net::{IpAddr, Ipv4Addr};

pub const PIAZZA_ADDR: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10001);

pub mod converter;
pub mod ifc;
pub mod ifc_grpc;
