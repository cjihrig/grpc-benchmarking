extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern crate grpc;

use std::thread;

mod echo_grpc;
mod echo;

struct EchoServiceImpl;

impl echo_grpc::EchoService for EchoServiceImpl {
    fn echo(
        &self,
        _o: grpc::RequestOptions,
        _req: echo::EchoRequest,
    ) -> grpc::SingleResponse<echo::EchoResponse> {
        let mut r = echo::EchoResponse::new();
        r.set_string_value(String::from("hello"));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let port = 3000;

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_addr(("127.0.0.1", port)).expect("set_addr");
    server.add_service(echo_grpc::EchoServiceServer::new_service_def(EchoServiceImpl));
    let _server = server.build().expect("server build");

    println!(
        "echo server started on port {}",
        port,
    );

    loop {
        thread::park();
    }
}
