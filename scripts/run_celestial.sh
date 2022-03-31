#!/bin/bash

PID=$$
TSLEEP=0.01


if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    exit 1
fi

rm -f data/bodies.dat
# cat data/sun_earth.dat > data/bodies.dat
rm -f data/temp_bodies.dat

# initialize random bodies
cargo run --bin circular -q  2>/dev/null > data/temp_bodies.dat
mv data/temp_bodies.dat data/bodies.dat

TOP=$1
writedata() {
    for i in $(seq $TOP); do
        cargo run --bin circular -q 2>/dev/null > data/temp_bodies.dat 
        mv data/temp_bodies.dat data/bodies.dat
        sleep $TSLEEP
    done

}

(writedata; kill -TERM -$$) &
sleep $TSLEEP
gnuplot plt/nbody.plt

