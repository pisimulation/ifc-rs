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

pub trait ProxyService {
    fn proxy(&self, o: ::grpc::RequestOptions, p: super::ifc::ProxyRequest) -> ::grpc::SingleResponse<super::ifc::ProxyResponse>;
}

// client

pub struct ProxyServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_proxy: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ifc::ProxyRequest, super::ifc::ProxyResponse>>,
}

impl ::grpc::ClientStub for ProxyServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ProxyServiceClient {
            grpc_client: grpc_client,
            method_proxy: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ifc.ProxyService/proxy".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl ProxyService for ProxyServiceClient {
    fn proxy(&self, o: ::grpc::RequestOptions, p: super::ifc::ProxyRequest) -> ::grpc::SingleResponse<super::ifc::ProxyResponse> {
        self.grpc_client.call_unary(o, p, self.method_proxy.clone())
    }
}

// server

pub struct ProxyServiceServer;


impl ProxyServiceServer {
    pub fn new_service_def<H : ProxyService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ifc.ProxyService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ifc.ProxyService/proxy".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.proxy(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait VCPiazzaService {
    fn post_msg(&self, o: ::grpc::RequestOptions, p: super::ifc::PostPayload) -> ::grpc::SingleResponse<super::ifc::PostResponse>;

    fn see_board(&self, o: ::grpc::RequestOptions, p: super::ifc::FetchPayload) -> ::grpc::SingleResponse<super::ifc::FetchResponse>;
}

// client

pub struct VCPiazzaServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_post_msg: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ifc::PostPayload, super::ifc::PostResponse>>,
    method_see_board: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ifc::FetchPayload, super::ifc::FetchResponse>>,
}

impl ::grpc::ClientStub for VCPiazzaServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        VCPiazzaServiceClient {
            grpc_client: grpc_client,
            method_post_msg: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ifc.VCPiazzaService/post_msg".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_see_board: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ifc.VCPiazzaService/see_board".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl VCPiazzaService for VCPiazzaServiceClient {
    fn post_msg(&self, o: ::grpc::RequestOptions, p: super::ifc::PostPayload) -> ::grpc::SingleResponse<super::ifc::PostResponse> {
        self.grpc_client.call_unary(o, p, self.method_post_msg.clone())
    }

    fn see_board(&self, o: ::grpc::RequestOptions, p: super::ifc::FetchPayload) -> ::grpc::SingleResponse<super::ifc::FetchResponse> {
        self.grpc_client.call_unary(o, p, self.method_see_board.clone())
    }
}

// server

pub struct VCPiazzaServiceServer;


impl VCPiazzaServiceServer {
    pub fn new_service_def<H : VCPiazzaService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ifc.VCPiazzaService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ifc.VCPiazzaService/post_msg".to_string(),
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
                        name: "/ifc.VCPiazzaService/see_board".to_string(),
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
