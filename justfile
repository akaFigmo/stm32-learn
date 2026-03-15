objdump:
    cargo objdump --bin stm32-learn -- -h

readobj:
    cargo readobj --bin stm32-learn -- -x .text

build:
    cargo clean
    cargo build