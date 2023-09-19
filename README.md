```
Rust:

siege -c 50 -t1 http://localhost:1337

Lifting the server siege...
Transactions:		       32824 hits
Availability:		       99.86 %
Elapsed time:		       60.96 secs
Data transferred:	        0.56 MB
Response time:		        0.02 secs
Transaction rate:	      538.45 trans/sec
Throughput:		        0.01 MB/sec
Concurrency:		       12.71
Successful transactions:       32824
Failed transactions:	          45
Longest transaction:	       19.51
Shortest transaction:	        0.00

siege -c 10 -t1 http://localhost:1337

Lifting the server siege...
Transactions:		       32744 hits
Availability:		       99.97 %
Elapsed time:		       60.82 secs
Data transferred:	        0.56 MB
Response time:		        0.00 secs
Transaction rate:	      538.38 trans/sec
Throughput:		        0.01 MB/sec
Concurrency:		        2.57
Successful transactions:       32744
Failed transactions:	           9
Longest transaction:	       19.51
Shortest transaction:	        0.00


Go

siege -c 50 -t1 http://localhost:1338

Lifting the server siege...
Transactions:		       32848 hits
Availability:		       99.87 %
Elapsed time:		       60.30 secs
Data transferred:	        0.56 MB
Response time:		        0.03 secs
Transaction rate:	      544.74 trans/sec
Throughput:		        0.01 MB/sec
Concurrency:		       17.28
Successful transactions:       32848
Failed transactions:	          43
Longest transaction:	       19.51
Shortest transaction:	        0.00

siege -c 50 -t5 http://localhost:1338

Lifting the server siege...
Transactions:		      163975 hits
Availability:		       99.89 %
Elapsed time:		      300.46 secs
Data transferred:	        2.81 MB
Response time:		        0.06 secs
Transaction rate:	      545.75 trans/sec
Throughput:		        0.01 MB/sec
Concurrency:		       34.22
Successful transactions:      163975
Failed transactions:	         175
Longest transaction:	       21.18
Shortest transaction:	        0.00

siege -c 10 -t1 http://localhost:1338

Lifting the server siege...
Transactions:		       32754 hits
Availability:		       99.98 %
Elapsed time:		       60.05 secs
Data transferred:	        0.56 MB
Response time:		        0.01 secs
Transaction rate:	      545.45 trans/sec
Throughput:		        0.01 MB/sec
Concurrency:		        4.69
Successful transactions:       32754
Failed transactions:	           7
Longest transaction:	       19.52
Shortest transaction:	        0.00

```
