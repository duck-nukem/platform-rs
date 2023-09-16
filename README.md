# Installation

Use the [justfile](https://github.com/casey/just) to perform certain operations

https://cheatography.com/linux-china/cheat-sheets/justfile/

`just --list`

# Project aims

- No javascript (or as minimal as possible)
- Secure!
- Full SSR, optinally with HTMX
- Browser native only (alerts, HTML5 elements, etc)
- Max performance & minimum resource requirements

Last recorded performance: 

* ~1k rps with sessions in postgres
* ~5k rps  with sessions only in cookies
* ~20k rps without database access;

Fresh django with postgres only doing a `select 1`; to measure perf:

```
$ oha "http://localhost:8080/healthcheck/" -z 5s -c 150 --disable-compression
Summary:
  Success rate:	0.9975
  Total:	5.0023 secs
  Slowest:	0.7122 secs
  Fastest:	0.1653 secs
  Average:	0.2977 secs
  Requests/sec:	488.1723

  Total data:	5.36 MiB
  Size/request:	2.25 KiB
  Size/sec:	1.07 MiB

Response time histogram:
  0.165 [1]   |
  0.220 [171] |■■■■■
  0.275 [827] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.329 [937] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.384 [294] |■■■■■■■■■■
  0.439 [88]  |■■■
  0.493 [68]  |■■
  0.548 [17]  |
  0.603 [8]   |
  0.658 [14]  |
  0.712 [11]  |

Latency distribution:
  10% in 0.2269 secs
  25% in 0.2515 secs
  50% in 0.2929 secs
  75% in 0.3241 secs
  90% in 0.3518 secs
  95% in 0.4370 secs
  99% in 0.6122 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0044 secs, 0.0017 secs, 0.0064 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 2388 responses
  [500] 48 responses

Error distribution:
  [6] connection error: Connection reset by peer (os error 54)
```

running the same command for `platform-rs`:

```
$ ./perf_test.sh
Summary:
  Success rate:	0.9992
  Total:	5.0020 secs
  Slowest:	0.0897 secs
  Fastest:	0.0103 secs
  Average:	0.0318 secs
  Requests/sec:	4700.3505

  Total data:	20.75 MiB
  Size/request:	926 B
  Size/sec:	4.15 MiB

Response time histogram:
  0.010 [1]     |
  0.018 [36]    |
  0.026 [5011]  |■■■■■■■■■■■■■
  0.034 [12135] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.042 [4455]  |■■■■■■■■■■■
  0.050 [888]   |■■
  0.058 [466]   |■
  0.066 [246]   |
  0.074 [141]   |
  0.082 [77]    |
  0.090 [37]    |

Latency distribution:
  10% in 0.0239 secs
  25% in 0.0267 secs
  50% in 0.0302 secs
  75% in 0.0345 secs
  90% in 0.0402 secs
  95% in 0.0476 secs
  99% in 0.0676 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0038 secs, 0.0016 secs, 0.0044 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 23493 responses

Error distribution:
  [18] connection error: Connection reset by peer (os error 54)
```