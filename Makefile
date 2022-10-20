build:
	# ... build ...
	cargo +stable build
	cd jsondata-derive; cargo +stable build
	cargo +nightly build
	cd jsondata-derive; cargo +nightly build
	#
	# ... test ...
	cargo +stable test --no-run
	cd jsondata-derive; cargo +stable test --no-run
	cargo +nightly test --no-run
	cd jsondata-derive; cargo +nightly test --no-run
	#
	# ... bench ...
	cargo +nightly bench --no-run
	cd jsondata-derive; cargo +nightly bench --no-run
	#
	# ... doc ...
	cargo +stable doc
	cd jsondata-derive; cargo +stable bench --no-run
	cargo +nightly doc
	cd jsondata-derive; cargo +nightly bench --no-run
	#
	# ... meta commands ...
	cargo +nightly clippy --all-targets --all-features
	cd jsondata-derive; cargo +nightly clippy --all-targets --all-features

test:
	# ... test stable ...
	cargo +stable test
	cd jsondata-derive; cargo +stable test
	cargo +stable run --example macro
	cargo +stable run --example mixed_integer
	# ... test nightly ...
	cargo +nightly test
	cd jsondata-derive; cargo +nightly test
	cargo +nightly run --example macro
	cargo +nightly run --example mixed_integer

bench:
	# ... bench stable ...
	# TODO: cargo +stable bench
	# TODO: cd jsondata-derive; cargo +stable bench
	# ... bench nightly ...
	cargo +nightly bench
	cd jsondata-derive; cargo +nightly bench

flamegraph:
	echo "not an executable"

prepare: build test bench
	check.sh check.out
	perf.sh perf.out

clean:
	cargo clean
	rm -f check.out perf.out flamegraph.svg perf.data perf.data.old
