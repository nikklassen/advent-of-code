#!/bin/bash
read -r a
n=${#a}
nb2=$(($n / 2))
nm1=$(($n-1))
for (( i = 0; i <= $nm1; i++ )); do
    j=$((($i + nb2) % n))
    if [[ ${a:$i:1} == ${a:$j:1} ]]; then
        sum=$(($sum + ${a:$i:1}))
    fi
done
echo $sum
