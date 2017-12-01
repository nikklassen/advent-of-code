#!/bin/bash
read -r a
n=${#a}
nm1=$(($n-1))
for (( i = 0; i <= $nm1; i++ )); do
    j=$((($i + 1) % n))
    if [[ ${a:$i:1} == ${a:$j:1} ]]; then
        sum=$(($sum + ${a:$i:1}))
    fi
done
echo $sum
