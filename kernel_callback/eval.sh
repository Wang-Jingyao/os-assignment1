#! /bin/bash

mkdir -p target

set -euo pipefail

token=$(cat /proc/sys/kernel/random/uuid)

pushd user || exit 1
cp eval-template.S eval.S
sed -i "s/Token:/Token: $token/g" eval.S

riscv64-unknown-elf-gcc -static -nostdlib ./eval.S -o ../target/eval
popd || exit 1

cargo osdk run --target-arch=riscv64 | tee "target/out.log"

match=$(grep -c "$token" target/out.log)
if [ "$match" -ne "0" ]
then
    echo "Success"
    exit 0
else
    echo "Failed"
    exit 1
fi  

