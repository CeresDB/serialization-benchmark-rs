# Serialization-benchmark-rs
Benchmark for serialization in Rust, https://github.com/apache/incubator-horaedb/issues/1515.

# Related projects
- https://github.com/CeresDB/hash-benchmark-rs


# Usage
```sh
[]# serialization-benchmark-rs --help
Usage: serialization-benchmark-rs [OPTIONS]

Options:
      --batch-size <BATCH_SIZE>  [env: BATCH_SIZE=] [default: 1000000]
      --unsafe                   use unsafe feature within flatbuffer [env: UNSAFE=]
  -h, --help                     Print help
  -V, --version                  Print version


[]# ./serialization-benchmark-rs 
Benchmark test, batch_size=1000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.049742587(s) | 0.2990468(s)     | 27.905426   |
| fury       | 0.4078921(s)   | 0.31441632(s)    | 26.46583    |
| protobuf   | 0.22009465(s)  | 0.3065254(s)     | 28.004875   |
+------------+----------------+------------------+-------------+

[]# ./serialization-benchmark-rs --batch-size 5000000
Benchmark test, batch_size=5000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.21562137(s)  | 1.4061759(s)     | 29.822226   |
| fury       | 2.0300736(s)   | 1.6155167(s)     | 25.778494   |
| protobuf   | 1.065279(s)    | 1.5093864(s)     | 25.547447   |
+------------+----------------+------------------+-------------+

[]# ./serialization-benchmark-rs --enable-unsafe
Benchmark test, batch_size=1000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.055235952(s) | 0.02805682(s)    | 29.246042   |
| fury       | 0.4071525(s)   | 0.31315032(s)    | 26.696833   |
| protobuf   | 0.216035(s)    | 0.30267346(s)    | 25.301205   |
+------------+----------------+------------------+-------------+
```

