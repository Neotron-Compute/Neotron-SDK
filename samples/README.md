# Neotron Sample Applications

Here are some sample applications that use the Neotron API.

## Building the Applications

Build the application as follows:

```console
$ cargo build --release --target=thumbv6m-none-eabi
$ cp ./target/thumbv6m-none-eabi/release/hello /my/sdcard/hello.elf
```

Then copy the resulting `hello.elf` file to an SD card and insert it into your Neotron system. You can load the application with something like:

```text
> load hello.elf
> run
```

## List of Sample Applications

## [`hello`](./hello)

This is a basic "Hello World" application. It prints the string "Hello, world" to *standard output* and then exits with an exit code of 0.

## [`input-test`](./input-test)

This reports any bytes received on Standard Input. Press Ctrl-X to quit.

## [`panic`](./panic)

This application panics, printing a nice panic message.

## [`fault`](./fault)

This application generates a Hard Fault.

## [`asmhello`](./asmhello)

A basic "Hello, world" but written in ARM assembly. It prints the string "Hello, world" to *standard output* and then exits with an exit code of 0.

## [`chello`](../chello)

A basic "Hello, world" but written in C. It prints the string "Hello, world" to *standard output* and then exits with an exit code of 0.
