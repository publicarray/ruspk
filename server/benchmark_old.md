# Unscientific Benchmark

* Not an entirely fair benchmark
* Benchmarked on a 2012 Macbook Pro
* Results are in requests/sec
* `Connection: Close` because keep-alive requests are unrealistic as a benchmark for the real world.
* without `Connection: Close` because I want to see what is possible.

|| with `Connection: Close` | without `Connection: Close`|
|-|-|-|
|nginx + php-fpm (valet)|545|2740|
|docker + hpgy/spkrepo|46|68|
|[spkrepo](https://github.com/SynoCommunity/spkrepo/pull/44#issuecomment-632007107)|1|1|
|[spkrepo + CACHE_TYPE="simple"](https://github.com/SynoCommunity/spkrepo/pull/44#issuecomment-632007107)|221|221|
|docker + sspks|547|1961|
|warp (hello world)|547|38248|
|actix-web (hello world) |547|44947|
|ruspk - rocket & mariadb (diesel)|529||
|ruspk - actix-web & mariadb (diesel)|547|1131|
|ruspk - actix-web & sqlite (diesel)|548|646|
|ruspk - actix-web & postgres (diesel)|404|435|
|ruspk - actix-web & postgres Full Copy (diesel) |133|140|
|ruspk - actix-web & postgres (diesel) & in-memory cache |547|31619|
|ruspk 0.1.4 - actix-web, postgres, in-memory cache, TTL |547|21767|

So postgresql uses more CPU than any other DB tested here (the queries are probably not optimised for it)

<https://www.postgresql.org/docs/12/pgbench.htm>

```sh
$ pgbench -i ruspk
$ pgbench -c 8 -j 10 -T 30 -f db/pg-bench.sql ruspk
starting vacuum...end.
transaction type: pq.sql
scaling factor: 1
query mode: simple
number of clients: 8
number of threads: 8
duration: 30 s
number of transactions actually processed: 15075
latency average = 15.931 ms
tps = 502.154301 (including connections establishing)
tps = 502.403372 (excluding connections establishing)
```

## nginx

```sh
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost'
Running 30s test @ http://localhost
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    31.92ms   15.40ms 158.45ms   76.39%
    Req/Sec   189.92     94.70   595.00     81.13%
  Latency Distribution
     50%   28.76ms
     75%   39.58ms
     90%   50.26ms
     99%   81.64ms
  16403 requests in 30.07s, 7.27MB read
  Socket errors: connect 0, read 39, write 0, timeout 0
  Non-2xx or 3xx responses: 16403
Requests/sec:    545.52
Transfer/sec:    247.72KB
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

$ hey -z 30s -disable-keepalive -h2 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Summary:
  Total: 31.1365 secs
  Slowest: 1.2191 secs
  Fastest: 0.0365 secs
  Average: 1.0645 secs
  Requests/sec: 46.1195

  Total data: 94220269 bytes
  Size/request: 65613 bytes
```

### docker + sspks

```sh
$ docker run --rm -d --name sspks \
    -v /Users/seb/git/spksrc/packages:/home/user/gosspks/packages/:rw \
    -p 9999:8080 \
    -e GOSSPKS_HOSTNAME=localhost \
    jdel/gosspks:v0.1

$ wrk -c 100 -t 8 -d 30 'http://localhost:9999/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:9999/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    45.06ms   26.55ms 407.52ms   79.46%
    Req/Sec   271.20     68.86     1.79k    80.82%
  59030 requests in 30.09s, 7.83MB read
  Socket errors: connect 0, read 0, write 0, timeout 96
Requests/sec:   1961.47
Transfer/sec:    266.25KB

$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:9999/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:9999/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   112.14ms  118.30ms   1.13s    96.10%
    Req/Sec    82.92     42.14   222.00     63.20%
  Latency Distribution
     50%   90.57ms
     75%  108.56ms
     90%  132.70ms
     99%  877.79ms
  16470 requests in 30.10s, 2.48MB read
Requests/sec:    547.13
Transfer/sec:     84.42KB
```

# ruspk

## warp

```
$ wrk --latency -c 100 -t 8 -d 30 'http://localhost:3030/hello/warp'
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

$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:3030/hello/warp'
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
$ wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/hello/warp'
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

$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/hello/warp'
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
$ wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
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


$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
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

## [actix-web](http://actix.rs/) & sqlite

```
$ wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   147.93ms   50.56ms 375.47ms   57.32%
    Req/Sec    81.15     17.82   150.00     62.56%
  Latency Distribution
     50%  118.08ms
     75%  198.78ms
     90%  211.68ms
     99%  235.00ms
  19452 requests in 30.11s, 35.19MB read
Requests/sec:    646.12
Transfer/sec:      1.17MB


$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   142.01ms   47.36ms 591.72ms   73.95%
    Req/Sec    82.32     21.65   161.00     66.87%
  Latency Distribution
     50%  137.11ms
     75%  166.50ms
     90%  199.70ms
     99%  286.28ms
  16490 requests in 30.08s, 30.13MB read
Requests/sec:    548.21
Transfer/sec:      1.00MB
```

## [actix-web](http://actix.rs/) & postgres

```
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   232.94ms   84.16ms 708.77ms   68.28%
    Req/Sec    51.20     20.42   120.00     64.74%
  Latency Distribution
     50%  223.42ms
     75%  285.45ms
     90%  345.05ms
     99%  461.14ms
  12189 requests in 30.10s, 22.27MB read
Requests/sec:    404.96
Transfer/sec:    757.72KB


$ wrk --latency -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   218.80ms   76.45ms 440.09ms   60.75%
    Req/Sec    54.82     21.21   120.00     64.56%
  Latency Distribution
     50%  178.93ms
     75%  293.18ms
     90%  318.73ms
     99%  368.03ms
  13122 requests in 30.10s, 23.74MB read
Requests/sec:    435.94
Transfer/sec:    807.60KB
```

## [actix-web](http://actix.rs/) & postgres (full copy of spkrepo)

```
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1'
Running 30s test @ http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   711.98ms  205.58ms   1.51s    68.66%
    Req/Sec    18.71     12.72   101.00     80.30%
  Latency Distribution
     50%  682.62ms
     75%  839.39ms
     90%    1.00s
     99%    1.26s
  3998 requests in 30.06s, 121.76MB read
Requests/sec:    133.00
Transfer/sec:      4.05MB
```

## [actix-web](http://actix.rs/) & postgres (full copy of spkrepo) & in-memory cache

```sh
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1'
Running 30s test @ http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    19.57ms   20.26ms 330.64ms   94.76%
    Req/Sec   231.55    127.46   787.00     77.47%
  Latency Distribution
     50%   15.44ms
     75%   22.83ms
     90%   32.68ms
     99%   61.80ms
  16439 requests in 30.05s, 500.64MB read
Requests/sec:    547.05
Transfer/sec:     16.66MB

$ wrk --latency -c 100 -t 8 -d 30 'http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1'
Running 30s test @ http://127.0.0.1:8080/?package_update_channel=beta&build=900&language=enu&major=6&micro=2&arch=x86&minor=1
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.95ms   44.27ms 897.51ms   98.42%
    Req/Sec     4.02k   394.61     5.24k    79.65%
  Latency Distribution
     50%    2.98ms
     75%    3.21ms
     90%    3.45ms
     99%  236.51ms
  948891 requests in 30.01s, 28.20GB read
Requests/sec:  31619.74
Transfer/sec:      0.94GB
```

---

## Different hardware

```sh
$ cargo install ruspk --features postgres
$ RUST_LOG="warn" CACHE_TTL=600 ruspk
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.89ms    5.61ms 212.02ms   95.17%
    Req/Sec     4.48k   833.09     8.38k    72.63%
  Latency Distribution
     50%    1.72ms
     75%    3.50ms
     90%    6.02ms
     99%   16.43ms
  1068415 requests in 30.10s, 64.06GB read
Requests/sec:  35499.86
Transfer/sec:      2.13GB

$ docker run --rm -it --name sspks \
          -v /home/seb/git/spksrc/packages:/home/user/gosspks/packages/:rw \
          -p 8080:8080 \
          -e GOSSPKS_HOSTNAME=localhost \
          jdel/gosspks:v0.1
$ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://localhost:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    28.16ms   24.74ms 277.29ms   83.32%
    Req/Sec   488.44    363.97     1.39k    75.50%
  Latency Distribution
     50%   19.43ms
     75%   39.60ms
     90%   61.06ms
     99%  112.50ms
  116618 requests in 30.07s, 637.38MB read
Requests/sec:   3878.72
Transfer/sec:     21.20MB

$ RUST_LOG="info" CACHE_TTL=600 ruspk
$ siege -c 100 -r 1000 --benchmark 'http://127.0.0.1:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Transactions:        100000 hits
Availability:        100.00 %
Elapsed time:         62.57 secs
Data transferred:      6127.74 MB
Response time:          0.06 secs
Transaction rate:      1598.21 trans/sec
Throughput:         97.93 MB/sec
Concurrency:         97.70
Successful transactions:      100000
Failed transactions:            0
Longest transaction:         0.93
Shortest transaction:         0.00

$ RUST_LOG="warn" CACHE_TTL=1 ruspk
$ siege -c 100 -r 1000 --benchmark 'http://127.0.0.1:8080/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Transactions:        100000 hits
Availability:        100.00 %
Elapsed time:          8.29 secs
Data transferred:      6127.74 MB
Response time:          0.01 secs
Transaction rate:     12062.73 trans/sec
Throughput:        739.17 MB/sec
Concurrency:         82.01
Successful transactions:      100000
Failed transactions:            0
Longest transaction:         0.15
Shortest transaction:         0.00
```

## spkrepo vs ruspk with the same database

* ruspk can out perform spkrepo by 10 times when memory cache is enabled!

```sh
## spkrepo wsgi NO Cache
# Every request hits the database as shown by the latency
# SPKREPO_CONFIG=$PWD/spkrepo/config-bench.py gunicorn -w (nproc) 'wsgi:app'
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.06s   359.55ms   1.46s    58.33%
    Req/Sec     7.46      7.38    38.00     69.41%
  Latency Distribution
     50%    1.33s
     75%    1.40s
     90%    1.46s
     99%    1.46s
  500 requests in 30.10s, 35.09MB read
  Socket errors: connect 0, read 0, write 0, timeout 476
Requests/sec:     16.61
Transfer/sec:      1.17MB

## spkrepo wsgi FileCache
# SPKREPO_CONFIG=$PWD/spkrepo/config-bench.py gunicorn -w (nproc) 'wsgi:app'
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    40.70ms   66.81ms 974.90ms   97.87%
    Req/Sec   377.73     47.94     0.94k    83.84%
  Latency Distribution
     50%   30.50ms
     75%   32.57ms
     90%   38.19ms
     99%  467.00ms
  88391 requests in 30.10s, 6.06GB read
Requests/sec:   2936.85
Transfer/sec:    206.08MB

## ruspk No Cache
# Every request hits the database as shown by the latency
# RUST_LOG="warn" CACHE_TTL=0  ./target/release/ruspk
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/nas?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://localhost:8080/nas?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   257.09ms   96.09ms 679.99ms   65.41%
    Req/Sec    46.58     19.30   120.00     69.10%
  Latency Distribution
     50%  249.51ms
     75%  322.56ms
     90%  388.30ms
     99%  493.87ms
  11157 requests in 30.06s, 685.06MB read
Requests/sec:    371.13
Transfer/sec:     22.79MB

## rusk memory cache
# RUST_LOG="warn" ./target/release/ruspk
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://localhost:8080/nas?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://localhost:8080/nas?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.58ms    2.16ms  84.95ms   91.06%
    Req/Sec     7.86k   617.07    11.84k    73.46%
  Latency Distribution
     50%    0.93ms
     75%    1.99ms
     90%    3.52ms
     99%   10.25ms
  1881521 requests in 30.10s, 112.82GB read
Requests/sec:  62514.48
Transfer/sec:      3.75GB
```
