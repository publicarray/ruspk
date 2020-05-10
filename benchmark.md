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

with [rocket.rs](https://rocket.rs)

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
