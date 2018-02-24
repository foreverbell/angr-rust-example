# angr-rust-example

Proof of concept that [angr](http://angr.io) may can analyze Rust binary. I post
it here in case of being useful to someone else.

## Usage

```bash
$ rustc --version
rustc 1.25.0-nightly (b1f8e6fb0 2018-02-22)
$ make
$ make solve
```

Be patient, and you will eventually see something like this within ~5 minutes.

```bash
(2710, <SimulationManager with 4 deferred, 4 deadended, 1 active>, 5)
h1baby
```
