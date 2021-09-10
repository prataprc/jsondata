build:
	# ... build ...
	cargo +nightly build
	# TODO: cargo +stable build
	# ... test ...
	cargo +nightly test --no-run
	# TODO: cargo +stable test --no-run
	# ... bench ...
	cargo +nightly bench --no-run
	# ... doc ...
	cargo +nightly doc
	# TODO: cargo +stable doc
	# ... meta commands ...
	cargo +nightly clippy --all-targets --all-features

test:
	# ... test ...
	cargo +nightly test
	cd jsondata-derive; cargo +nightly test
	cargo +nightly run --example macro
	# TODO: cargo +stable test --no-run
	# TODO: cd jsondata-derive; cargo +stable test
	# TODO: cargo +stable run --example macro

bench:
	# ... test ...
	cargo +nightly bench
	cd jsondata-derive; cargo +nightly bench
	# TODO: cargo +stable test --no-run
	# TODO: cd jsondata-derive; cargo +stable test

flamegraph:
	echo "not an executable"

prepare:
	check.sh check.out
	perf.sh perf.out

clean:
	cargo clean
	rm -f check.out perf.out flamegraph.svg perf.data perf.data.old
