# spkrepo vs ruspk Benchmark

* same database
* Not an entirely fair benchmark when it come to caching
  * spkrepo is using disk cache and ruspk is using memory
* `Connection: Close` because keep-alive requests are unrealistic as a benchmark for the real world.
* ruspk out performs spkrepo by a huge margin

||spkrepo|ruspk|
|-|-|-|
|No Cache|16 Req/sec|**371** Req/sec|
|With Cache|2,936 Req/sec|**62,514** Req/sec|

```sh
## spkrepo wsgi NO Cache
# Every request hits the database
# uncomment CACHE_TYPE = "NullCache" in config-bench.py
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

## ruspk No Cache
# Every request hits the database
# RUST_LOG="warn" CACHE_TTL=0 ./target/release/ruspk
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

## spkrepo wsgi FileSystemCache
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
