'use strict';
const Path = require('path');
const { Server, ServerCredentials } = require(process.env.GRPC_SERVER_DIR ||
                                              'grpc-server-js');
const { loadProtoFile } = require('./grpc-common');
const protoFile = Path.join(__dirname, 'echo.proto');
const { EchoService } = loadProtoFile(protoFile);
const server = new Server();

process.on('unhandledRejection', (err) => {
  throw err;
});

server.addService(EchoService.service, {
  echo (call, callback) {
    callback(null, { string_value: 'hello' });
  }
});

(async () => {
  await server.bind('localhost:3000', ServerCredentials.createInsecure());
  server.start();
})();
