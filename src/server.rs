use grpc;
use sharedlib::ifc::*;
use sharedlib::ifc_grpc::*;
use sharedlib::*;
use std::{io::Read, net::SocketAddr, thread};

struct ProxyServer;

impl ProxyService for ProxyServer {
    fn proxy(
        &self,
        _m: grpc::RequestOptions,
        req: ProxyRequest,
    ) -> grpc::SingleResponse<ProxyResponse> {
        println!(
            "[Proxy Server] got req: \nvc = {}\ncommand = {}\nlabel = {:#?}\n",
            req.get_vc(),
            req.get_command(),
            req.get_label()
        );
        let mut r = ProxyResponse::new();
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let users = vec![PI_ADDR, AMIT_ADDR];
    let vcs = vec![VC_PIAZZA_ADDR];
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(ProxyServiceServer::new_service_def(ProxyServer));
    server_builder
        .http
        .set_addr(SocketAddr::from(PROXY_SERVER_ADDR));
    let server = server_builder.build().expect("build");

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}
