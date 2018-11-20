# rust-client

A fast, opinionated command line HTTP client.

Fundamentally, `rust-client` is a thin wrapper around Rust's fantastic `reqwest` library. Unlike `curl`, however, it is designed more as a debugging tool. Headers are displayed above the response body, the command line interface is more intuitive than remembering flags, and default en-/decoding behavior.

## Performance

The following is a totally unscientific benchmark using `/usr/bin/time` to finely measure memory usage and timing for `rust-client`, `curl`, and Python `http`; on my very old development box:

```sh
> /usr/bin/time -l rc get localhost:8000
GET http://localhost:8000/
HTTP/1.1 200 OK
content-length: 9
date: Tue, 10 Jul 2018 14:21:55 GMT
---
It works!
        0.01 real         0.00 user         0.00 sys
   7290880  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1841  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
        10  messages sent
        10  messages received
         0  signals received
         4  voluntary context switches
        85  involuntary context switches

> /usr/bin/time -l curl localhost:8000
It works!        0.02 real         0.00 user         0.00 sys
   4956160  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
      1259  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
        10  messages sent
        10  messages received
         0  signals received
         7  voluntary context switches
        28  involuntary context switches

> /usr/bin/time -l http localhost:8000
HTTP/1.1 200 OK
content-length: 9
date: Tue, 10 Jul 2018 14:24:20 GMT

It works!

        0.62 real         0.41 user         0.18 sys
  26628096  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     26378  page reclaims
         0  page faults
         0  swaps
         0  block input operations
         0  block output operations
        10  messages sent
        10  messages received
        25  signals received
        77  voluntary context switches
       524  involuntary context switches
```

Note, the measurement of `http` is after warming up the Python runtime using multiple runs of `http`. The following is the initial result:

```sh
HTTP/1.1 200 OK
content-length: 9
date: Tue, 10 Jul 2018 14:23:53 GMT

It works!

        3.06 real         0.37 user         0.21 sys
  26618280  maximum resident set size
         0  average shared memory size
         0  average unshared data size
         0  average unshared stack size
     25678  page reclaims
       684  page faults
         0  swaps
       133  block input operations
         0  block output operations
        11  messages sent
        11  messages received
        25  signals received
       669  voluntary context switches
       675  involuntary context switches
```

While `rust-client` and `curl` perform similarly, `curl` does not print the same level of information that `rust-client` does.

The test server is the example server from `hyper`'s documentation.

## TODO
- Flag to disable ansi escaping for output
- HTML pretty printing
- encode body content in a specific format (JSON, YAML, etc)
- decode response based on Content-Type