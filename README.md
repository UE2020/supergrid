# supergrid
A simple, optimized, safe spatial hash grid.
# Benchmark
A benchmarking tool is included in the examples folder.
```
> RUSTFLAGS='-C target-cpu=native' cargo build --release --example bench
> ./target/release/examples/bench --cell-size 9 --count 10000 --width 100000 --height 100000 --min-size 10 --max-size 1000
Setup:
	Arena width:  100,000
	Arena height: 100,000
	Cell size:    512x512
	Entity count: 10,000
	Minimum entity size:  10x10
	Maximum entity size:  1000x1000
Took 1.866101ms to insert 10,000 entities
Took 3.663191ms to probe 10,000 entities
Collisions: 32,900; average: 3.29
```
