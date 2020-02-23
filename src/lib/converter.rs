use crate::ifc as grpc;

#[derive(Debug, Clone)]
pub struct PostPayload {
    pub msg: String,
    pub author: String,
    pub public: bool,
    pub anon: bool,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct FetchPayload {
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct PostResponse {
    pub msg_id: u32,
}

#[derive(Debug, Clone)]
pub struct FetchResponse {
    pub msg_board: String,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub secrecy: bool,
}

impl PostPayload {
    pub fn from_grpc(req: &grpc::PostPayload) -> Self {
        PostPayload {
            msg: req.get_msg().to_owned(),
            author: req.get_author().to_owned(),
            public: req.get_public(),
            anon: req.get_anon(),
            label: Label::from_grpc(req.get_label()),
        }
    }

    pub fn to_grpc(&self) -> grpc::PostPayload {
        let mut req = grpc::PostPayload::new();
        req.set_msg(self.msg.to_owned());
        req.set_author(self.author.to_owned());
        req.set_public(self.public);
        req.set_anon(self.anon);
        req.set_label(self.label.to_grpc());
        req
    }
}

impl FetchPayload {
    pub fn from_grpc(req: &grpc::FetchPayload) -> Self {
        FetchPayload {
            label: Label::from_grpc(req.get_label()),
        }
    }

    pub fn to_grpc(&self) -> grpc::FetchPayload {
        let mut req = grpc::FetchPayload::new();
        req.set_label(self.label.to_grpc());
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
            msg_board: req.get_msg_board().to_owned(),
            label: Label::from_grpc(req.get_label()),
        }
    }

    pub fn to_grpc(&self) -> grpc::FetchResponse {
        let mut req = grpc::FetchResponse::new();
        req.set_msg_board(self.msg_board.to_owned());
        req.set_label(self.label.to_grpc());
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
