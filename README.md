## Instructions

For generating load, [`ghz`](https://github.com/bojand/ghz) is used. For the purposes of this README, it is assumed that `ghz` is installed in the `$PATH`.

Start a server on port 3000:

```
$ node server-grpc-js.js
... or
$ node server-grpc.js
... or
$ ./bin/go-grpc-{arch}
```

Gather all the datas:

```
$ ghz -proto ./echo.proto -insecure -z 40s -c 100 -call EchoService/Echo -d '{}' localhost:3000
```


## Results

These are the results on macOS Mojave running on a late 2016 MBP with:
- 2.7 GHz Intel Core i7
- 16 GB 2133 MHz LPDDR3

### Node.js server-grpc-js

```
Summary:
  Count:	427266
  Total:	40006.10 ms
  Slowest:	52.00 ms
  Fastest:	0.96 ms
  Average:	9.32 ms
  Requests/sec:	10680.02

Response time histogram:
  0.962 [1]	|
  6.066 [23826]	|∎∎∎
  11.170 [325308]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  16.273 [66550]	|∎∎∎∎∎∎∎∎
  21.377 [9938]	|∎
  26.480 [1226]	|
  31.584 [242]	|
  36.688 [11]	|
  41.791 [64]	|
  46.895 [0]	|
  51.999 [100]	|

Latency distribution:
  10% in 6.79 ms
  25% in 7.66 ms
  50% in 8.77 ms
  75% in 10.39 ms
  90% in 12.58 ms
  95% in 14.73 ms
  99% in 18.28 ms
Status code distribution:
  [OK]	427266 responses
```

### Node.js server-grpc

```
Summary:
  Count:	471199
  Total:	40004.82 ms
  Slowest:	7419.06 ms
  Fastest:	1.84 ms
  Average:	8.45 ms
  Requests/sec:	11778.55

Response time histogram:
  1.841 [1]	|
  743.563 [471098]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  1485.284 [0]	|
  2227.006 [0]	|
  2968.727 [0]	|
  3710.448 [0]	|
  4452.170 [0]	|
  5193.891 [0]	|
  5935.613 [0]	|
  6677.334 [0]	|
  7419.055 [100]	|

Latency distribution:
  10% in 5.13 ms
  25% in 5.78 ms
  50% in 6.43 ms
  75% in 7.16 ms
  90% in 8.46 ms
  95% in 9.62 ms
  99% in 12.50 ms
Status code distribution:
  [OK]	471199 responses
```

### Go

```
Summary:
  Count:	2221650
  Total:	40002.44 ms
  Slowest:	48.80 ms
  Fastest:	0.19 ms
  Average:	1.76 ms
  Requests/sec:	55537.86

Response time histogram:
  0.193 [1]	|
  5.054 [991298]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  9.915 [7403]	|
  14.776 [837]	|
  19.637 [168]	|
  24.498 [174]	|
  29.359 [25]	|
  34.220 [1]	|
  39.081 [0]	|
  43.941 [44]	|
  48.802 [49]	|

Latency distribution:
  10% in 1.15 ms
  25% in 1.31 ms
  50% in 1.53 ms
  75% in 1.83 ms
  90% in 2.36 ms
  95% in 3.21 ms
  99% in 4.91 ms
Status code distribution:
  [OK]	2221650 responses
```
