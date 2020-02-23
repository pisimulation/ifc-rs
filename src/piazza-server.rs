use grpc;
use sharedlib::ifc::*;
use sharedlib::ifc_grpc::*;
use sharedlib::*;
use std::{collections::HashMap, net::SocketAddr, thread};

struct PiazzaServer {
    next_id: u32,
    db: Board,
}

struct Board(HashMap<u32, Post>);

impl Board {
    fn make_board(board: Board) -> String {}
}

struct Post {
    msg: String,
    author: String,
    public: bool,
    anon: bool,
    label: bool,
}

impl Post {
    fn make_post(post: Post) -> String {
        let author = if (post.anon) {
            "Anonymous".to_owned()
        } else {
            post.author
        };
        let pretty_post = concat!(
            "\nAuthor: ",
            author,
            "\nMsg: ",
            post.msg,
            "\npublic: ",
            post.public,
            "\nlabel",
            post.label
        );
        pretty_post
    }
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
        println!(
            "[VC Piazza] got post_msg: \nmsg = {}\npublic = {:#?}\nanon = {}\nlabel = {:#?}\n",
            req.get_msg(),
            req.get_public(),
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
        r.set_msg_board(Board::make_board(self.db));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.add_service(PiazzaServiceServer::new_service_def(PiazzaServer {
        next_id: 0,
        db: Board(HashMap::new()),
    }));
    server_builder.http.set_addr(SocketAddr::from(PIAZZA_ADDR));
    server_builder.build().expect("build");
    // Blocks the main thread forever
    loop {
        thread::park();
    }
}
