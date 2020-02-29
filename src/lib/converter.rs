use crate::ifc as grpc;

#[derive(Debug, Clone)]
pub struct PostPayload {
    pub post: Post,
}

#[derive(Debug, Clone)]
pub struct FetchPayload {}

#[derive(Debug, Clone)]
pub struct PostResponse {
    pub msg_id: u32,
}

#[derive(Debug, Clone)]
pub struct FetchResponse {
    pub msg_board: Board,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub msg: String,
    pub author: String,
    pub public: bool,
    pub anon: bool,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub secrecy: bool,
}

impl PostPayload {
    pub fn from_grpc(req: &grpc::PostPayload) -> Self {
        PostPayload {
            post: Post::from_grpc(req.get_post()),
        }
    }

    pub fn to_grpc(&self) -> grpc::PostPayload {
        let mut req = grpc::PostPayload::new();
        req.set_post(self.post.to_grpc());
        req
    }
}

impl FetchPayload {
    pub fn from_grpc(req: &grpc::FetchPayload) -> Self {
        FetchPayload {}
    }

    pub fn to_grpc(&self) -> grpc::FetchPayload {
        let mut req = grpc::FetchPayload::new();
        req
    }
}

impl PostResponse {
    pub fn from_grpc(req: &grpc::PostResponse) -> Self {
        PostResponse {
            msg_id: req.get_msg_id(),
        }
    }

    pub fn to_grpc(&self) -> grpc::PostResponse {
        let mut req = grpc::PostResponse::new();
        req.set_msg_id(self.msg_id);
        req
    }
}

impl FetchResponse {
    pub fn from_grpc(req: &grpc::FetchResponse) -> Self {
        FetchResponse {
            msg_board: Board::from_grpc(req.get_msg_board()),
            label: Label::from_grpc(req.get_label()),
        }
    }

    pub fn to_grpc(&self) -> grpc::FetchResponse {
        let mut req = grpc::FetchResponse::new();
        req.set_msg_board(self.msg_board.to_grpc());
        req.set_label(self.label.to_grpc());
        req
    }
}

impl Board {
    pub fn from_grpc(req: &grpc::Board) -> Self {
        Board {
            posts: req.get_posts().iter().map(Post::from_grpc).collect(),
        }
    }

    pub fn to_grpc(&self) -> grpc::Board {
        let mut req = grpc::Board::new();
        req.set_posts(protobuf::RepeatedField::from_vec(
            self.posts.iter().map(|post| post.to_grpc()).collect(),
        ));
        req
    }
}

impl Post {
    pub fn from_grpc(req: &grpc::Post) -> Self {
        Post {
            msg: req.get_msg().to_owned(),
            author: req.get_author().to_owned(),
            public: req.get_public(),
            anon: req.get_anon(),
        }
    }

    pub fn to_grpc(&self) -> grpc::Post {
        let mut req = grpc::Post::new();
        req.set_msg(self.msg.to_owned());
        req.set_author(self.author.to_owned());
        req.set_public(self.public);
        req.set_anon(self.anon);
        req
    }
}

impl Label {
    pub fn from_grpc(req: &grpc::Label) -> Self {
        Label {
            secrecy: req.get_secrecy(),
        }
    }

    pub fn to_grpc(&self) -> grpc::Label {
        let mut req = grpc::Label::new();
        req.set_secrecy(self.secrecy);
        req
    }
}
