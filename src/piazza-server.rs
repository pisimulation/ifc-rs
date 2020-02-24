use grpc;
use sharedlib;
use sharedlib::ifc::*;
use sharedlib::ifc_grpc::*;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};

struct PiazzaServer {
    next_id: u32,
    db: Arc<Mutex<sharedlib::converter::Board>>, // TODO: store this on MongoDB
}

// impl fmt::Debug for Post {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         writeln!(
//             f,
//             "\nauthor = {}\nmsg = {})\npublic = {}\nanon = {}\nlabel = {}\n",
//             self.author, self.msg, self.public, self.anon, self.label
//         )
//     }
// }

impl PiazzaService for PiazzaServer {
    fn post_msg(
        &self,
        _m: grpc::RequestOptions,
        req: PostPayload,
    ) -> grpc::SingleResponse<PostResponse> {
        (*self.db.lock().unwrap())
            .posts
            .push(sharedlib::converter::Post::from_grpc(req.get_post()));
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
        r.set_msg_board(self.db.lock().unwrap().to_grpc());
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(PiazzaServiceServer::new_service_def(PiazzaServer {
        next_id: 0,
        db: Arc::new(Mutex::new(sharedlib::converter::Board {
            posts: Vec::new(),
        })),
    }));
    server_builder
        .http
        .set_addr(SocketAddr::from(sharedlib::PIAZZA_ADDR));
    server_builder.build().expect("build");
    // Blocks the main thread forever
    loop {
        thread::park();
    }
}
