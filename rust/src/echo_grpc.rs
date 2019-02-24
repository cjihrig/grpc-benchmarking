// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

pub trait EchoService {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::echo::EchoRequest) -> ::grpc::SingleResponse<super::echo::EchoResponse>;
}

// client

pub struct EchoServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Echo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::echo::EchoRequest, super::echo::EchoResponse>>,
}

impl ::grpc::ClientStub for EchoServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        EchoServiceClient {
            grpc_client: grpc_client,
            method_Echo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/EchoService/Echo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl EchoService for EchoServiceClient {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::echo::EchoRequest) -> ::grpc::SingleResponse<super::echo::EchoResponse> {
        self.grpc_client.call_unary(o, p, self.method_Echo.clone())
    }
}

// server

pub struct EchoServiceServer;


impl EchoServiceServer {
    pub fn new_service_def<H : EchoService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/EchoService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/EchoService/Echo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.echo(o, p))
                    },
                ),
            ],
        )
    }
}
