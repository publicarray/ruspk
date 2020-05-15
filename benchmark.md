## Preliminary Benchmark

Not an entirely fair benchmark

### nginx

`Connection: Close` because ruspk and spkrepo do not have keep-alive

```sh
wrk --latency -H 'Connection: Close' -c 5k -t 10 -d 30 'http://localhost:80/'
Running 30s test @ http://localhost:80/
  10 threads and 5000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    55.54ms   33.51ms 386.69ms   81.55%
    Req/Sec   105.38     90.23   616.00     74.74%
  Latency Distribution
     50%   48.76ms
     75%   67.46ms
     90%   88.20ms
     99%  184.35ms
  15477 requests in 30.07s, 6.86MB read
  Socket errors: connect 2455, read 2709, write 0, timeout 0
  Non-2xx or 3xx responses: 15477
Requests/sec:    514.68
Transfer/sec:    233.72KB
```

### ruspk

with [rocket.rs](https://rocket.rs) & mariadb

```sh
$ wrk --latency -H 'Connection: Close' -c 5k -t 10 -d 30 'http://localhost:80/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:80/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  10 threads and 5000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    81.82ms   58.59ms 258.59ms   78.23%
    Req/Sec   102.95     76.67   616.00     77.00%
  Latency Distribution
     50%   65.03ms
     75%  118.65ms
     90%  179.66ms
     99%  231.50ms
  15941 requests in 30.10s, 42.34MB read
  Socket errors: connect 2455, read 2395, write 0, timeout 2
Requests/sec:    529.57
Transfer/sec:      1.41MB
```

### docker + spkrepo

```sh
$ docker run --name spkrepo --rm -it -p 8080:5000 hpgy/spkrepo:add_docker
$ wrk --latency -c 5k -t 10 -d 30 'http://localhost:8080/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  10 threads and 5000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.50s   281.37ms   1.99s    79.79%
    Req/Sec    11.15      8.87    50.00     83.45%
  Latency Distribution
     50%    1.56s
     75%    1.69s
     90%    1.76s
     99%    1.97s
  2054 requests in 30.09s, 0.88MB read
  Socket errors: connect 2455, read 1473, write 0, timeout 273
  Non-2xx or 3xx responses: 2054
Requests/sec:     68.27
Transfer/sec:     30.00KB
```

# ruspk

## warp

```
wrk --latency -c 100 -t 8 -d 30 'http://localhost:3030/hello/warp'
Running 30s test @ http://localhost:3030/hello/warp
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.51ms  406.62us  14.93ms   80.26%
    Req/Sec     4.81k   223.82     5.55k    78.83%
  Latency Distribution
     50%    2.41ms
     75%    2.67ms
     90%    3.01ms
     99%    3.79ms
  1147851 requests in 30.01s, 141.21MB read
Requests/sec:  38248.11
Transfer/sec:      4.71MB

ruspk on  actix-web 📝is 📦 v0.1.0 via 🦀 v1.45.0-nightly
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:3030/hello/warp'
Running 30s test @ http://localhost:3030/hello/warp
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.72ms   23.94ms 316.10ms   97.87%
    Req/Sec   249.92    144.73   848.00     82.76%
  Latency Distribution
     50%   20.73ms
     75%   29.65ms
     90%   37.00ms
     99%  118.98ms
  16450 requests in 30.04s, 2.02MB read
  Socket errors: connect 0, read 16450, write 0, timeout 0
Requests/sec:    547.66
Transfer/sec:     68.99KB
```

## actix-web

```
ruspk on  actix-web 📝is 📦 v0.1.0 via 🦀 v1.45.0-nightly
❯ wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/hello/warp'
Running 30s test @ http://localhost:8080/hello/warp
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.13ms  283.56us  11.68ms   82.09%
    Req/Sec     5.65k   444.00     7.68k    76.08%
  Latency Distribution
     50%    2.09ms
     75%    2.26ms
     90%    2.42ms
     99%    2.79ms
  1348809 requests in 30.01s, 164.65MB read
Requests/sec:  44947.09
Transfer/sec:      5.49MB

ruspk on  actix-web 📝is 📦 v0.1.0 via 🦀 v1.45.0-nightly
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/hello/warp'
Running 30s test @ http://localhost:8080/hello/warp
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.89ms   14.56ms 168.47ms   91.69%
    Req/Sec   233.46    148.06   848.00     79.31%
  Latency Distribution
     50%   12.17ms
     75%   19.36ms
     90%   28.47ms
     99%   68.91ms
  16447 requests in 30.02s, 2.31MB read
Requests/sec:    547.87
Transfer/sec:     78.65KB
```

## [actix-web](http://actix.rs/) & mariadb

```
wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    85.29ms   33.48ms 472.11ms   76.82%
    Req/Sec   142.31     38.58   260.00     68.31%
  Latency Distribution
     50%   76.54ms
     75%   99.66ms
     90%  129.91ms
     99%  197.62ms
  34044 requests in 30.10s, 99.74MB read
Requests/sec:   1131.11
Transfer/sec:      3.31MB


wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   100.40ms   42.18ms 455.16ms   80.01%
    Req/Sec    75.67     36.31   203.00     69.04%
  Latency Distribution
     50%   94.31ms
     75%  117.53ms
     90%  143.95ms
     99%  252.30ms
  16466 requests in 30.09s, 48.54MB read
Requests/sec:    547.23
Transfer/sec:      1.61MB
```
