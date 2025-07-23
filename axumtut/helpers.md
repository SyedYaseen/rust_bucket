cargo watch -q -c -w src/ -x run
cargo watch -q -c -w testserver/ -x "test -q quick_dev -- --nocapture"