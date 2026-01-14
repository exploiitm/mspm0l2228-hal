TARGET=MSPM0L222X

all: patch generate format

patch:
	svdtools patch svd/$(TARGET).yml svd/$(TARGET).patched.svd

generate:
	svd2rust --target=cortex-m --reexport-core-peripherals -i svd/$(TARGET).patched.svd -o . --edition=2024
	rm -r src
	form -i lib.rs -o src/
	rm lib.rs

format:
	cargo fmt
