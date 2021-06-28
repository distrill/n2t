#! /bin/bash

for filename in $(ls data | grep .tst | grep -v VME); do
	file=data/$filename
	vmfile="data/$(basename $file .tst).vm"
	echo building ${vmfile}
	cargo run $vmfile
	echo testing $file
	CPUEmulator.sh $file
	echo ""
done
