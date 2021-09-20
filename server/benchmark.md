# spkrepo vs ruspk Benchmark

* same database
* Not an entirely fair benchmark when it come to caching
  * spkrepo is using disk cache and ruspk is using memory
* `Connection: Close` because keep-alive requests are unrealistic as a benchmark for the real world.
* ruspk out performs spkrepo by a huge margin

||spkrepo|ruspk|
|-|-|-|
|No Cache|16 Req/sec|**371** Req/sec|
|With build in Memory Cache||**62,514** Req/sec|
|With File Cache|3,231 Req/sec||
|With Redis Cache|2,779 Req/sec||

To run this test on your own, my spkrepo [config-bench.py](https://github.com/publicarray/spkrepo/blob/benchmark/spkrepo/config-bench.py)

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
    Latency    38.02ms   67.84ms 904.39ms   97.85%
    Req/Sec   415.64     34.48   474.00     85.67%
  Latency Distribution
     50%   28.74ms
     75%   29.07ms
     90%   29.53ms
     99%  475.55ms
  97108 requests in 30.05s, 6.65GB read
Requests/sec:   3231.25
Transfer/sec:    226.74MB

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

## spkrepo wsgi RedisCache
# SPKREPO_CONFIG=$PWD/spkrepo/config-bench.py gunicorn -w (nproc) 'wsgi:app'
❯ wrk --latency -H 'Connection: Close' -c 100 -t 8 -d 30 'http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2'
Running 30s test @ http://127.0.0.1:8000/nas/?package_update_channel=beta&build=24922&language=enu&major=6&micro=2&arch=x86_64&minor=2
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    34.41ms    4.35ms  81.63ms   95.48%
    Req/Sec   349.34     34.03   393.00     92.85%
  Latency Distribution
     50%   33.56ms
     75%   34.36ms
     90%   36.00ms
     99%   58.46ms
  83674 requests in 30.10s, 5.73GB read
Requests/sec:   2779.90
Transfer/sec:    195.07MB

```
