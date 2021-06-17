#!/bin/bash

######################
#  Made by MrZloHex  #
#     16.06.2021     #
######################

MAN_PATH="/usr/local/man/man1"
NAME="emuBOOB"
COMP_PATH="./target/release/"

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
	if [ -f "${COMP_PATH}${NAME}" ]
	then
		echo >&6 "Compiling finished succesfully"
	else
		echo >&6 "Compiling failed"
		exit 1
	fi
}

install() {
	echo >&6 -n "Installing"

	load &
	PID=$!

	sudo cp "${COMP_PATH}${NAME}" /usr/local/bin

	sudo mkdir $MAN_PATH
	sudo cp $NAME.1 $MAN_PATH
	sudo gzip $MAN_PATH/$NAME.1

	sleep 1	
	echo >&6 ""

	kill $PID
}

uninstall() {
	echo >&6 -n "Uninstalling"

	load &
	PID=$!

	sudo rm /usr/local/bin/$NAME
	sudo rm $MAN_PATH/$NAME.1

	sleep 1
	echo >&6 ""

	kill $PID
}

main() {
	exec 6>&1 >/dev/null
	exec 2>/dev/null

	sudo ls


	case $1 in
		"-i" | "--install") 
			compile
			check_exec
			install
			echo >&6 "DONE!!!"
			;;
		"-u" | "--uninstall")
			uninstall
			echo >&6 "DONE!!!"
			;;
		*)
			echo >&6 "For installation run:"
			echo >&6 "	./deployment.sh -i"
			echo >&6 "	./deployment.sh --install"
			echo >&6 "For uninstallation run:"
			echo >&6 "	./deployment.sh -u"
			echo >&6 "	./deployment.sh --uninstall"
			;;
	esac
}

main $@
