#!/bin/bash

compile() {
	echo >&6 -n "Compiling"

	load &
	PID=$!
	echo >&6 ""

	kill $PID
}

main() {
	exec 6>&1 >/dev/null
	
	compile
}

# FOR ON ENTRY ROOT PASSWORD
sudo ls

main $@
