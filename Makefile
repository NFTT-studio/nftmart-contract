build:
	cd example && proxychains cargo +nightly contract build

test:
	cd example && proxychains cargo +nightly test
