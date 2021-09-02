#! /usr/bin/env bash

exec > $1
exec 2>&1

set -o xtrace

PERF=$HOME/.cargo/target/release/perf

date; time cargo +nightly bench -- --nocapture || exit $?
# TODO: date; time cargo +stable bench -- --nocapture || exit $?

date; valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes cargo +nightly test --release -- --nocapture || exit $?
date; valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes cargo +nightly test -- --nocapture || exit $?
date; valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes cargo +nightly bench -- --nocapture || exit $?
