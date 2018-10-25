package main

import (
	"log"
	"net"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

type echoServer struct {}

func (s *echoServer) Echo(ctx context.Context, req *EchoRequest) (*EchoResponse, error) {
	return &EchoResponse{
		StringValue: "hello",
	}, nil
}

func newEchoServer() *echoServer {
	s := &echoServer{}
	return s
}

func main() {
	lis, err := net.Listen("tcp", "localhost:3000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	RegisterEchoServiceServer(grpcServer, newEchoServer())
	grpcServer.Serve(lis)
}
