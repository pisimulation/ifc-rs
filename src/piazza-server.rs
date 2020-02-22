use grpc;
use sharedlib::ifc::*;
use sharedlib::ifc_grpc::*;
use sharedlib::*;
use std::{net::SocketAddr, thread};

struct PiazzaServer;

impl PiazzaService for PiazzaServer {
    fn post_msg(
        &self,
        _m: grpc::RequestOptions,
        req: PostPayload,
    ) -> grpc::SingleResponse<PostResponse> {
        println!(
            "[VC Piazza] got post_msg: \nmsg = {}\naudience = {:#?}\nanon = {}\nlabel = {:#?}\n",
            req.get_msg(),
            req.get_audience(),
            req.get_anon(),
            req.get_label()
        );
        let mut r = PostResponse::new();
        grpc::SingleResponse::completed(r)
    }

    fn see_board(
        &self,
        _m: grpc::RequestOptions,
        req: FetchPayload,
    ) -> grpc::SingleResponse<FetchResponse> {
        println!("[VC Piazzza] Got see_board");
        let mut r = FetchResponse::new();
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let users = vec![PI_ADDR, AMIT_ADDR];
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(PiazzaServiceServer::new_service_def(PiazzaServer));
    server_builder.http.set_addr(SocketAddr::from(PIAZZA_ADDR));
    let server = server_builder.build().expect("build");
    // Blocks the main thread forever
    loop {
        thread::park();
    }
}
