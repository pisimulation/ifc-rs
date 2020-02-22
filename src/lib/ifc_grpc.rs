// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait PiazzaService {
    fn post_msg(&self, o: ::grpc::RequestOptions, p: super::ifc::PostPayload) -> ::grpc::SingleResponse<super::ifc::PostResponse>;

    fn see_board(&self, o: ::grpc::RequestOptions, p: super::ifc::FetchPayload) -> ::grpc::SingleResponse<super::ifc::FetchResponse>;
}

// client

pub struct PiazzaServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_post_msg: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ifc::PostPayload, super::ifc::PostResponse>>,
    method_see_board: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ifc::FetchPayload, super::ifc::FetchResponse>>,
}

impl ::grpc::ClientStub for PiazzaServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PiazzaServiceClient {
            grpc_client: grpc_client,
            method_post_msg: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ifc.PiazzaService/post_msg".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_see_board: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ifc.PiazzaService/see_board".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl PiazzaService for PiazzaServiceClient {
    fn post_msg(&self, o: ::grpc::RequestOptions, p: super::ifc::PostPayload) -> ::grpc::SingleResponse<super::ifc::PostResponse> {
        self.grpc_client.call_unary(o, p, self.method_post_msg.clone())
    }

    fn see_board(&self, o: ::grpc::RequestOptions, p: super::ifc::FetchPayload) -> ::grpc::SingleResponse<super::ifc::FetchResponse> {
        self.grpc_client.call_unary(o, p, self.method_see_board.clone())
    }
}

// server

pub struct PiazzaServiceServer;


impl PiazzaServiceServer {
    pub fn new_service_def<H : PiazzaService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ifc.PiazzaService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ifc.PiazzaService/post_msg".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.post_msg(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ifc.PiazzaService/see_board".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.see_board(o, p))
                    },
                ),
            ],
        )
    }
}
