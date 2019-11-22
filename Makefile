.PHONY:
all: libusdt
	RUSTFLAGS="-L$(PWD)/libusdt -lusdt" cargo build

.PHONY: libusdt
libusdt:
	$(MAKE) -C libusdt

.PHONY: clean
clean:
	cargo clean
	$(MAKE) -C libusdt clean
