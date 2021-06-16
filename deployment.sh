#!/bin/bash

NAME="emuBOOB"

load() {
	while [ 1 ]
	do
		echo >&6 -ne "."
		sleep 0.5
	done
}

compile() {
	echo >&6 -n "Compiling"

	load &
	PID=$!
	
	cargo build --release

	echo >&6 ""

	kill $PID
}

check_exec() {
	if [ -f "./target/release/$NAME" ]
	then
		echo >&6 "Compiling finished succesfully"
	else
		echo >&6 "Compiling failed"
	fi
}

main() {
	exec 6>&1 >/dev/null
	exec 2>/dev/null

	sudo ls

	compile
	check_exec


}

main $@
