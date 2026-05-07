# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd fuzz && cargo +nightly fuzz build main_libfuzzer && popd" && \
    mv fuzz/target/x86_64-unknown-linux-gnu/release/main_libfuzzer /main_libfuzzer && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd fuzz && cargo +nightly build --release --bin main_libfuzzer && popd" && \
    mv fuzz/target/release/main_libfuzzer /main_libfuzzer_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /main_libfuzzer /main_libfuzzer_no_inst /
