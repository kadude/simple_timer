.PHONY: install clean
# .DEFAULT_GOAL := generate
.DEFAULT_GOAL := install

BIN = ~/olescripts
INSTALL_TARGET = ${BIN}/simpleTimer
TARGET = target/release/simple_rust_timer

install: $(TARGET) ${INSTALL_TARGET}

$(TARGET): src/**.rs
	@echo "Bulding cargo release..."
	cargo build --release

${INSTALL_TARGET}: $(TARGET)
	@echo "Copying to ${INSTALL_TARGET}..."
	mkdir -p ${BIN}
	cp $(TARGET) ${INSTALL_TARGET}

clean:
	@echo "Removing ${INSTALL_TARGET}..."
	rm ${INSTALL_TARGET}