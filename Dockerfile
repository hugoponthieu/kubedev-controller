FROM rust:1.98.0-trixie AS builder

ARG WORKDIR="/controller"

WORKDIR ${WORKDIR}

COPY . .

RUN cargo build --release --bin kubedev-controller

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime

COPY --from=builder /controller/target/release/kubedev-controller / 

ENTRYPOINT [ "/kubedev-controller" ]
