'use strict';
const Path = require('path');
const { Server, ServerCredentials } = require('grpc');
const { loadProtoFile } = require('./grpc-common');
const protoFile = Path.join(__dirname, 'echo.proto');
const { EchoService } = loadProtoFile(protoFile);
const server = new Server();

server.addService(EchoService.service, {
  echo (call, callback) {
    callback(null, { string_value: 'hello' });
  }
});

server.bind('localhost:3000', ServerCredentials.createInsecure());
server.start();
