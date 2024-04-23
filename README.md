# Serialization-benchmark-rs
Benchmark for serialization in Rust, https://github.com/apache/incubator-horaedb/issues/1515.

# Related projects
- https://github.com/CeresDB/hash-benchmark-rs


# Usage
```sh
%> serialization-benchmark-rs --help
Usage: serialization-benchmark-rs [OPTIONS]

Options:
      --batch-size <BATCH_SIZE>  [env: BATCH_SIZE=] [default: 1000000]
  -h, --help                     Print help
  -V, --version                  Print version


%> cargo run
Benchmark test, batch_size=1000000, result: 

+------------+----------------+------------------+-------------+
| name       | serialize time | deserialize time | cpu_utility |
| flatbuffer | 0.32837144(s)  | 1.5406052(s)     | 11.836201   |
| fury       | 2.7501004(s)   | 2.738932(s)      | 20.819456   |
| protobuf   | 1.4705708(s)   | 2.3939092(s)     | 20.486391   |
+------------+----------------+------------------+-------------+
```
