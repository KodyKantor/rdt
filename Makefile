.PHONY:
all: libusdt
	cargo build

.PHONY: libusdt
libusdt:
	$(MAKE) -C libusdt

.PHONY: clean
clean:
	cargo clean
	$(MAKE) -C libusdt clean

.PHONY: test
test: all
	cargo test
