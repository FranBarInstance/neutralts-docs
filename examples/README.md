Neutral TS Examples
===================

This directory contains example implementations demonstrating how to use Neutral TS template system with various programming languages and frameworks.

Neutral TS is a web template system that allows using the same templates with any programming language.

## Available Examples

Some examples require the IPC server: [Neutral TS IPC Server](https://github.com/FranBarInstance/neutral-ipc/releases)

### Rust Examples
- **helloworld**: Basic "Hello World" example with Rust
- **pwa-actix**: PWA implementation with Actix web framework
- **pwa-axum**: PWA implementation with Axum web framework
- **pwa-warp**: PWA implementation with Warp web framework

### Python Examples
- **helloworld**: Basic "Hello World" example with Python
- **helloworldipc**: Hello World with IPC integration
- **pwa-flask**: PWA implementation with Flask

### PHP Examples
- **helloworld**: Basic "Hello World" example with PHP
- **pwa**: Progressive Web App implementation

### Go Examples
- **helloworld**: Basic "Hello World" example with Go
- **pwa**: Progressive Web App implementation

### Node.js Examples
- **helloworld**: Basic "Hello World" example with Node.js
- **pwa-express**: PWA implementation with Express.js

## Performance

The examples include performance benchmarks comparing different language implementations using the same Neutral TS templates. All templates are rendered in Rust regardless of the host language.

Tests performed on the PWA example for each language and have all the elements of a real case except for DB access, which does not exist in the examples.

Just to give a rough idea of the performance of Neutral TS for a real-life case.

The tests were performed with: CPU 6 cores 3000MHz. 32GB RAM

### Rust Actix PWA cache:

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   513.24us  120.15us   4.24ms   88.45%
    Req/Sec     9.67k   618.98    10.49k    75.74%
  194310 requests in 10.10s, 5.71GB read
Requests/sec:  19238.79
Transfer/sec:    579.10MB
```

### Rust Actix PWA no cache:

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.47ms  711.40us  15.31ms   67.69%
    Req/Sec     1.12k    98.42     1.33k    67.50%
  22360 requests in 10.00s, 673.05MB read
Requests/sec:   2235.33
Transfer/sec:     67.29MB
```

### PHP PWA cache:

Apache

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.00ms  163.88us   5.69ms   83.41%
    Req/Sec     4.99k   214.18     5.23k    77.72%
  100307 requests in 10.10s, 2.96GB read
Requests/sec:   9932.04
Transfer/sec:    299.79MB
```

### PHP PWA no cache:

Apache

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.97ms  581.76us  12.68ms   79.12%
    Req/Sec     1.01k    35.62     1.06k    69.50%
  20098 requests in 10.01s, 606.67MB read
Requests/sec:   2008.05
Transfer/sec:     60.61MB
```

### Python PWA cache:

Apache WSGI

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.42ms   10.03ms 149.65ms   98.68%
    Req/Sec     3.61k   309.23     3.85k    92.93%
  71292 requests in 10.00s, 2.10GB read
Requests/sec:   7128.99
Transfer/sec:    214.96MB
```

### Python PWA no cache:

Apache WSGI

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.45ms  572.12us  14.09ms   81.09%
    Req/Sec     0.92k    30.78     0.97k    71.50%
  18343 requests in 10.01s, 553.14MB read
Requests/sec:   1833.29
```

### Node.js PWA cache

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.25ms    1.07ms  24.42ms   83.15%
    Req/Sec   803.49     92.29     0.92k    78.00%
  16003 requests in 10.00s, 483.67MB read
Requests/sec:   1599.85
Transfer/sec:     48.35MB
```

### Node.js PWA no cache

```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.43ms    2.71ms  18.90ms   62.38%
    Req/Sec   481.53     45.26   626.00     71.50%
  9589 requests in 10.00s, 289.82MB read
Requests/sec:    958.49
Transfer/sec:     28.97MB
```

### Go PWA cache:
```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.79ms  411.90us   4.33ms   67.92%
    Req/Sec     2.79k    89.90     2.96k    73.00%
  55527 requests in 10.00s, 1.63GB read
Requests/sec:   5552.46
Transfer/sec:    167.31MB
```

### Go PWA no cache:
```
wrk http://localhost/
Running 10s test @ http://localhost/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.39ms  665.07us  13.85ms   78.27%
    Req/Sec     0.93k    37.72     0.99k    63.00%
  18530 requests in 10.00s, 558.36MB read
Requests/sec:   1852.51
Transfer/sec:     55.82MB
```

Links
-----

Neutral TS template engine.

- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples)
