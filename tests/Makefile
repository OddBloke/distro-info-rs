tests_dir := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

ci-install-deps:
	sudo apt-get install -qqy shunit2

apply:
	cat $(wildcard *.patch) | patch -p0

unapply:
	cat $(wildcard *.patch) | patch -R -p0

run-debian-tests:
	COMMAND=$(tests_dir)/../target/debug/debian-distro-info ./distro-info/test-debian-distro-info

run-ubuntu-tests:
	COMMAND=$(tests_dir)/../target/debug/ubuntu-distro-info ./distro-info/test-ubuntu-distro-info

test: apply run-ubuntu-tests unapply
