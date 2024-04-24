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
  -h, --help                     Print help
  -V, --version                  Print version


[]# ./serialization-benchmark-rs 
Benchmark test, batch_size=1000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.043271985(s) | 0.2820203(s)     | 30.636393   |
| fury       | 0.4075856(s)   | 0.3214576(s)     | 28.214052   |
| protobuf   | 0.2138659(s)   | 0.3004469(s)     | 25.609756   |
+------------+----------------+------------------+-------------+

[]# ./serialization-benchmark-rs --batch-size 5000000
Benchmark test, batch_size=5000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.21562137(s)  | 1.4061759(s)     | 29.822226   |
| fury       | 2.0300736(s)   | 1.6155167(s)     | 25.778494   |
| protobuf   | 1.065279(s)    | 1.5093864(s)     | 25.547447   |
+------------+----------------+------------------+-------------+
```

