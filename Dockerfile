FROM rust

COPY ./recursive-future-leak /usr/src/recursive-future-leak

RUN apt-get update ; apt-get install -y valgrind

WORKDIR /usr/src/recursive-future-leak

RUN cargo build

CMD valgrind --leak-check=full ./target/debug/recursive-future-leak
