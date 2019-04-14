#!/usr/bin/env bash
declare -i sum
declare -i inr
declare -i cnt
sum = 0
inr = 1
cnt = 1

for i in {1..4}
do
    cnt=$((cnt + inr))
    inr=$((inr + 2))
    sum=$((sum + cnt))

    echo "Square $i has $inr grains."
    echo "Sum is $sum"
    echo
done
