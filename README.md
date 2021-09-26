# Norwegian National ID

This is a Rust library for working with Norwegian national identification numbers.

## Example: Count FH-numbers

FH-numbers are national ID numbers starting with 8 and 9. To count all valid such numbers, run

    cargo run --release --example count_fh

To see that there are 165,289,000 such numbers.

# Profiling

To profile on OSX, do

    cargo build --release --example count_fh
    xcrun xctrace record --template 'Time Profiler' --output count_fh.trace --launch target/release/examples/count_fh
    open count_fh.trace