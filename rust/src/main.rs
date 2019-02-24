extern crate grpc;
extern crate futures;
extern crate httpbis;

use std::thread;

mod echo_grpc;
mod echo;


struct EchoServiceImpl {
    instance: u32,
}

impl echo_grpc::EchoService for EchoServiceImpl {
    fn echo(
        &self,
        _o: grpc::RequestOptions,
        _req: echo::EchoRequest,
    ) -> grpc::SingleResponse<echo::EchoResponse> {
        // println!("request on instance {}", self.instance);
        let mut r = echo::EchoResponse::new();
        r.set_string_value(format!("hello"));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut conf = httpbis::ServerConf::default();
    conf.reuse_port = Some(true);
    let port = 3000;

    let mut server1 = grpc::ServerBuilder::new_plain();
    server1.http.conf = conf.clone();
    server1.http.set_addr(("127.0.0.1", port)).expect("set_addr");
    server1.add_service(echo_grpc::EchoServiceServer::new_service_def(EchoServiceImpl { instance: 1 }));
    let _server1 = server1.build().expect("server 1");

    /*
    let mut server2 = grpc::ServerBuilder::new_plain();
    server2.http.conf = conf.clone();
    server2.http.set_addr(("127.0.0.1", port)).expect("set_addr");
    server2.add_service(echo_grpc::EchoServiceServer::new_service_def(EchoServiceImpl { instance: 2 }));
    let _server2 = server2.build().expect("server 2");
    */

    println!(
        "echo server started on port {}",
        port,
    );

    loop {
        thread::park();
    }
}
