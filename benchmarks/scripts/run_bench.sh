#!/bin/bash
#
# Benchmarks currently checked out Rust.

MYDIR=$(dirname $0)
source "$MYDIR/dirs.sh"

cd $RUST_DIR

echo "building"
./x.py build

# This is the old code that used to measure bootstrap time.  It is
# disabled because it has not been ported to work with rustbuild yet.
#
#export RUSTFLAGS_STAGE2="-Ztime-passes -Zinput-stats"
#for i in 0 1 2
#do
#    echo "building, round $i"
#    git show HEAD -s >$TIMES_DIR/raw/rustc--$DATE--$i.log
#    touch src/librustc_trans/lib.rs
#    make >>$TIMES_DIR/raw/rustc--$DATE--$i.log
#done
#echo "processing data"
#cd $TIMES_DIR
#python $SCRIPTS_DIR/process.py rustc $DATE 3
#for i in 0 1 2
#do
#    git add raw/rustc--$DATE--$i.log
#done
#git add processed/rustc--$DATE.json

echo "benchmarks"
export RUSTC_DIR=$START/build/x86_64-unknown-linux-gnu/stage2
export RUSTC=$RUSTC_DIR/bin/rustc
export LD_LIBRARY_PATH=$RUSTC_DIR/lib
cd $BENCH_DIR
./process.sh

echo "committing"
cd $TIMES_DIR
git commit -m "Added data for $DATE"
git push upstream master
