#!/bin/sh
set -e
awk -F, 'BEGIN {
printf "Network;Time, s;Speed, Mbit/s\n"
}
{
    printf "%s;%f;%f\n", $1, $2/1000, $3/($2/1000)/1024/1024
}' < benchmark.txt | column -t -s';'
