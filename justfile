objdump:
    cargo objdump -- -h target/thumbv7em-none-eabihf/debug/stm32-learn

readobj:
    cargo readobj -- -S target/thumbv7em-none-eabihf/debug/stm32-learn

build:
    cargo clean
    cargo build