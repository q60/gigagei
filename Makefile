NAME = gigagei
TARGET = target/release/gigagei

.PHONY: build test

all: build test

build:
	cargo build --release

test:
	$(TARGET)

install:
	install -Dm755 "$(TARGET)" "$(DESTDIR)/usr/bin/$(NAME)"
	install -Dm644 "license" "$(DESTDIR)/usr/share/licenses/$(NAME)/license"

uninstall:
	rm -rfv "$(DESTDIR)/usr/bin/$(NAME)" "$(DESTDIR)/usr/share/licenses/$(NAME)"
