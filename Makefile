all: build/main build/bench

build:
	mkdir -p $@

build/main: build/main.o build/libbase64.a
	$(CC) -o $@ $^ -lpthread -ldl

build/libbase64.a: src/base64.rs | build
	rustc src/base64.rs --crate-type staticlib -o $@

build/main.o: src/main.c | build
	$(CC) -o $@ -c $<

solve: build/main
	./solve.py main

build/bench: src/main.c src/cc-bench/base64.cc
	$(CC) -o $@ $^ -O2 -std=c++11 -lstdc++

bench: build/bench
	./solve.py bench

clean:
	rm -rf build
