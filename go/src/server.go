package main

import (
	pb "./echo.pb.go"
	"log"
	"net"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

type echoServer struct {}

func (s *echoServer) Echo(ctx context.Context, req *pb.EchoRequest) (*pb.EchoResponse, error) {
	return &pb.EchoResponse{
		string_value: "hello"
	}
}

func newEchoServer() *echoServer {
	s := &echoServer{}
	return s
}

func main() {
	lis, err := net.Listen("tcp", "0.0.0.0:3000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterEchoServiceServer(grpcServer, newEchoServer())
	grpcServer.Serve(lis)

}
