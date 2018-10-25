# grpc-benchmarking

For generating load, [`ghz`](https://github.com/bojand/ghz) is used. For the purposes of this README, it is assumed that `ghz` is installed in the `$PATH`.

Start a server on port 3000:

```
$ node server-grpc-js.js
... or
$ node server-grpc.js
```

Gather all the datas:

```
$ ghz -proto ./echo.proto -insecure -z 40s -c 100 -call EchoService/Echo -d '{}' localhost:3000
```
