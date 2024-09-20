# Simple Port Scanner

A simple, but fast asynchronous port scanner written in Rust. It is able scan all 65,535 ports in under 5s (just on localhost machine for now).

## Usage

Run the scanner with default settings (all ports):

```
simple-port-scanner.exe
```

Scan a specific range of ports:

```
simple-port-scanner.exe <start_port> <end_port>
```

Example:

```
simple-port-scanner.exe 0 5000
```

This will scan ports 0 through 5000.
