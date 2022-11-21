# Introduction

A set of rust project for learning purpose.

# Projects

1. Snake: a simple snake game
2. Chat-tokio-steam: chat app based on tokio



# Compile & Run

## Compile
```shell
> cargo build --release
```

## Run
### Snake

```shell
> ./target/release/snake
```
### chat-tokio-steam

#### start server
```shell
> ./target/release/chat-tokio-steam
Welcome to online chat stream!
```
#### start client

start client in a new terminal (support multiple client)
```shell
> telnet localhost 7373
```

![image-20221120162202196](./doc/img/chat-tokio-steam-demo.png)

