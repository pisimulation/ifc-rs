use grpc;
use label::dclabel::DCLabel;
use sharedlib;
use sharedlib::ifc::*;
use sharedlib::ifc_grpc::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};

// $ cargo run --bin piazza

struct PiazzaServer {
    next_id: Arc<Mutex<u32>>,
    db: Arc<Mutex<sharedlib::converter::Board>>, // TODO: store this on MongoDB
    taint: Arc<Mutex<DCLabel>>,
}

impl PiazzaService for PiazzaServer {
    fn post_msg(
        &self,
        _m: grpc::RequestOptions,
        req: PostPayload,
    ) -> grpc::SingleResponse<PostResponse> {
        println!("[Piazza] Got a post");
        (*self.db.lock().unwrap())
            .posts
            .push(sharedlib::converter::Post::from_grpc(req.get_post()));
        let mut r = PostResponse::new();
        r.set_msg_id(*self.next_id.lock().unwrap());
        *self.next_id.lock().unwrap() += 1;
        grpc::SingleResponse::completed(r)
    }

    fn see_board(
        &self,
        _m: grpc::RequestOptions,
        req: FetchPayload,
    ) -> grpc::SingleResponse<FetchResponse> {
        println!("[Piazzza] Got see_board");
        let mut r = FetchResponse::new();
        r.set_msg_board((*self.db.lock().unwrap()).to_grpc());
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(PiazzaServiceServer::new_service_def(PiazzaServer {
        next_id: Arc::new(Mutex::new(0)),
        db: Arc::new(Mutex::new(sharedlib::converter::Board {
            posts: Vec::new(),
        })),
        taint: Arc::new(Mutex::new(DCLabel::public())),
    }));
    server_builder
        .http
        .set_addr(SocketAddr::from(sharedlib::PIAZZA_ADDR));
    let server = server_builder.build().expect("build");
    println!("Piazza Service running {}", server.local_addr());

    // Blocks the main thread forever
    loop {
        thread::park();
    }
}
