# 测试

# v1

```bash
oha http://127.0.0.1:8080 -c 10 -z 10s
Summary:
Success rate: 100.00%
Total: 10.0010 secs
Slowest: 0.0045 secs
Fastest: 0.0004 secs
Average: 0.0008 secs
Requests/sec: 11953.5539

Total data: 1.60 MiB
Size/request: 14 B
Size/sec: 163.42 KiB

Response time histogram:
0.000 [1] |
0.001 [28680] |■■■■■■■■■■
0.001 [89827] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
0.002 [673] |
0.002 [253] |
0.002 [95] |
0.003 [7] |
0.003 [0] |
0.004 [2] |
0.004 [0] |
0.004 [1] |

Response time distribution:
10.00% in 0.0007 secs
25.00% in 0.0008 secs
50.00% in 0.0008 secs
75.00% in 0.0009 secs
90.00% in 0.0010 secs
95.00% in 0.0010 secs
99.00% in 0.0012 secs
99.90% in 0.0019 secs
99.99% in 0.0023 secs

Details (average, fastest, slowest):
DNS+dialup: 0.0001 secs, 0.0000 secs, 0.0034 secs
DNS-lookup: 0.0000 secs, 0.0000 secs, 0.0027 secs

Status code distribution:
[200] 119539 responses

Error distribution:
[8] aborted due to deadline
```

# v2.1

```bash
oha http://127.0.0.1:8080 -c 10 -z 10s
Summary:
  Success rate:	100.00%
  Total:	10.0008 secs
  Slowest:	0.0181 secs
  Fastest:	0.0006 secs
  Average:	0.0018 secs
  Requests/sec:	5570.4667

  Total data:	761.52 KiB
  Size/request:	14 B
  Size/sec:	76.15 KiB

Response time histogram:
  0.001 [1]     |
  0.002 [55301] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.004 [380]   |
  0.006 [8]     |
  0.008 [0]     |
  0.009 [0]     |
  0.011 [0]     |
  0.013 [0]     |
  0.015 [0]     |
  0.016 [0]     |
  0.018 [10]    |

Response time distribution:
  10.00% in 0.0016 secs
  25.00% in 0.0016 secs
  50.00% in 0.0018 secs
  75.00% in 0.0019 secs
  90.00% in 0.0020 secs
  95.00% in 0.0020 secs
  99.00% in 0.0023 secs
  99.90% in 0.0028 secs
  99.99% in 0.0181 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0001 secs, 0.0000 secs, 0.0021 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0021 secs

Status code distribution:
  [200] 55700 responses

Error distribution:
  [9] aborted due to deadline
```

# v2.2

```bash
oha http://127.0.0.1:8080 -c 10 -z 10s

Summary:
  Success rate:	100.00%
  Total:	10.0005 secs
  Slowest:	0.0041 secs
  Fastest:	0.0003 secs
  Average:	0.0006 secs
  Requests/sec:	16638.7936

  Total data:	2.38 MiB
  Size/request:	15 B
  Size/sec:	243.72 KiB

Response time histogram:
  0.000 [1]      |
  0.001 [117892] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.001 [47377]  |■■■■■■■■■■■■
  0.001 [869]    |
  0.002 [242]    |
  0.002 [4]      |
  0.003 [0]      |
  0.003 [2]      |
  0.003 [0]      |
  0.004 [1]      |
  0.004 [1]      |

Response time distribution:
  10.00% in 0.0005 secs
  25.00% in 0.0005 secs
  50.00% in 0.0006 secs
  75.00% in 0.0007 secs
  90.00% in 0.0007 secs
  95.00% in 0.0007 secs
  99.00% in 0.0009 secs
  99.90% in 0.0015 secs
  99.99% in 0.0017 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0001 secs, 0.0000 secs, 0.0029 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0028 secs

Status code distribution:
  [200] 166389 responses

Error distribution:
  [8] aborted due to deadline
```
