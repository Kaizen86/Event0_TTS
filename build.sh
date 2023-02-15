#!/bin/bash
date > buildtimes.txt
#export CARGO_LOG=trace

cargo build
date >> buildtimes.txt

cargo build --release
date >> buildtimes.txt

echo Build script done
cat buildtimes.txt
