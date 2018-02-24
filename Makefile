LDFLAGS := -lpthread -ldl

all: build/main

build:
	mkdir -p $@

build/main: build/main.o build/libbase64.a
	$(CC) -o $@ $^ $(LDFLAGS)

build/libbase64.a: src/base64.rs
	rustc src/base64.rs --crate-type staticlib -o $@

build/main.o: src/main.c | build
	$(CC) -o $@ -c $<

solve: build/main
	./solve.py

clean:
	rm -rf build
