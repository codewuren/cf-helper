#This is a makefile by CodeWuRen

build:
	cargo build

install:
	cargo build
	install ./target/debug/cf-helper "/usr/bin/cf"

uninstall:
	rm /usr/bin/cf
