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

To test that the service is working run the following:

```
$ grpcurl --plaintext --proto echo.proto localhost:3000 EchoService/Echo
```

## Results

These are the results on macOS Mojave running on a late 2016 MBP with:
- 2.7 GHz Intel Core i7
- 16 GB 2133 MHz LPDDR3

### CPU Usage

Using the [`sampleproc`](https://github.com/opendtrace/toolkit/blob/master/Proc/sampleproc) dtrace script the % of utilization of each process across all CPUs was determined, below are the results. Additionally, running the `topsysproc` script provided information about the number of syscalls for each during a 1 second period, which is also shown below.

| Process | % CPU Usage | # of Samples | Avg Syscall Count |
|---------|-------------|--------------|-------------------|
| node    | 14          | 4596         | 2500              |
| rust    | 14          | 4511         | 80000             |
| go      | 30          | 10079        | 120000            |

Below are htop results for each of the processe. As you can see, memory usage remained low for each process as well, with Node.js using the most.

#### Go htop

![go htop](images/go-htop.png)

#### Rust htop

![rust htop](images/rust-htop.png)


#### Node.js htop

![node htop](images/node-htop.png)

### Node.js server-grpc-js

```
Summary:
  Count:    507558
  Total:    40012.28 ms
  Slowest:    49.65 ms
  Fastest:    0.71 ms
  Average:    7.85 ms
  Requests/sec:    12685.06

Response time histogram:
  0.713 [1]    |
  5.607 [41278]    |∎∎∎∎
  10.500 [419461]    |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  15.394 [42988]    |∎∎∎∎
  20.287 [2781]    |
  25.181 [646]    |
  30.074 [126]    |
  34.968 [95]    |
  39.861 [82]    |
  44.755 [0]    |
  49.648 [100]    |

Latency distribution:
  10% in 5.84 ms
  25% in 6.62 ms
  50% in 7.27 ms
  75% in 8.90 ms
  90% in 10.32 ms
  95% in 11.71 ms
  99% in 14.51 ms
Status code distribution:
  [OK]    507558 responses
```

### Node.js server-grpc

```
Summary:
  Count:    568545
  Total:    40144.39 ms
  Slowest:    1981.89 ms
  Fastest:    2.32 ms
  Average:    7.02 ms
  Requests/sec:    14162.50

Response time histogram:
  2.323 [1]    |
  200.279 [568044]    |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  398.235 [200]    |
  596.191 [0]    |
  794.148 [0]    |
  992.104 [0]    |
  1190.060 [0]    |
  1388.016 [0]    |
  1585.973 [0]    |
  1783.929 [200]    |
  1981.885 [100]    |

Latency distribution:
  10% in 4.71 ms
  25% in 5.14 ms
  50% in 5.55 ms
  75% in 6.23 ms
  90% in 7.39 ms
  95% in 8.73 ms
  99% in 11.49 ms
Status code distribution:
  [OK]    568545 responses
```

### Go

Running with GOMAXPROCS=1

```
Summary:
  Count:	1742699
  Total:	40001.02 ms
  Slowest:	70.97 ms
  Fastest:	0.19 ms
  Average:	2.26 ms
  Requests/sec:	43566.37

Response time histogram:
  0.188 [1]	|
  7.267 [998072]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  14.345 [1313]	|
  21.423 [208]	|
  28.501 [107]	|
  35.579 [104]	|
  42.658 [54]	|
  49.736 [62]	|
  56.814 [43]	|
  63.892 [30]	|
  70.970 [6]	|

Latency distribution:
  10% in 1.58 ms
  25% in 1.91 ms
  50% in 2.17 ms
  75% in 2.47 ms
  90% in 2.99 ms
  95% in 3.37 ms
  99% in 4.78 ms
Status code distribution:
  [OK]	1742699 responses
```

### Rust

Compiled in release mode

```
Summary:
  Count:	806332
  Total:	40008.54 ms
  Slowest:	39.00 ms
  Fastest:	1.06 ms
  Average:	4.93 ms
  Requests/sec:	20153.99

Response time histogram:
  1.059 [1]	|
  4.853 [543524]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  8.647 [259033]	|∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  12.441 [3479]	|
  16.235 [44]	|
  20.029 [90]	|
  23.823 [48]	|
  27.617 [85]	|
  31.411 [0]	|
  35.205 [4]	|
  38.999 [24]	|

Latency distribution:
  10% in 4.23 ms
  25% in 4.39 ms
  50% in 4.60 ms
  75% in 5.14 ms
  90% in 6.29 ms
  95% in 6.50 ms
  99% in 7.45 ms
Status code distribution:
  [OK]	806332 responses
```
