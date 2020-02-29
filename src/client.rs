use grpc::ClientStub;
use sharedlib::converter::*;
use sharedlib::ifc_grpc::*;
use std::sync::Arc;

// $ cargo run --bin client

fn main() {
    // A post to add to Piazza board
    let post_req = PostPayload {
        post: Post {
            msg: "Question about HW 1".to_owned(),
            author: "Pi".to_owned(),
            public: true, // visible to everyone
            anon: true,   // author anonymous to classmates
        },
    };
    // A request to see the board
    let fetch_req = FetchPayload {};
    let grpc_client =
        Arc::new(grpc::Client::new_plain("127.0.0.1", 10001, Default::default()).unwrap());
    let student = PiazzaServiceClient::with_client(grpc_client);
    let post_res = student.post_msg(grpc::RequestOptions::new(), post_req.to_grpc());
    match post_res.wait() {
        Err(e) => panic!("Error: {:?}", e),
        Ok((_, value, _)) => println!("[Pi] POST RES {:?}", value),
    }

    let fetch_res = student.see_board(grpc::RequestOptions::new(), fetch_req.to_grpc());
    match fetch_res.wait() {
        Err(e) => panic!("Error: {}", e),
        Ok((_, value, _)) => println!("[Pi] FETCH RES {:?}", value),
    }
}
