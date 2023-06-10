# Neotron Sample Applications

Here are some sample applications that use the Neotron API.

## Building the Applications

Build the application as follows:

```console
$ cargo build --release --target=thumbv6m-none-eabi
$ rust-objcopy -O binary ./target/thumbv6m-none-eabi/release/hello hello.bin
```

Then copy the resulting `hello.bin` file to an SD card and insert it into your Neotron system. You can load the application with something like:

```text
> load hello.bin
> run
```

If you don't have `rust-objcopy` installed, install it with:

```console
$ rustup component add llvm-tools
```

## List of Sample Applications

## [`hello`](./hello)

This is a basic "Hello World" application. It prints the string "Hello, world" to *standard output* and then exits with an exit code of 0.

## [`panic`](./panic)

This application panics, printing a nice panic message.

## [`fault`](./fault)

This application generates a Hard Fault.
