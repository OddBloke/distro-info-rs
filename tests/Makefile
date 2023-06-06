tests_dir := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

apply:
	cat $(wildcard *.patch) | patch -p0

unapply:
	cat $(wildcard *.patch) | patch -R -p0

run-ubuntu-tests:
	COMMAND=$(tests_dir)/../target/debug/ubuntu-distro-info ./distro-info/test-ubuntu-distro-info

test: apply run-ubuntu-tests unapply
