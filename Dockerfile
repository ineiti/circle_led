FROM debian:bookworm-slim

RUN apt update && apt install -y patchelf ca-certificates

COPY target/dx/circle_led/release/web /web
RUN ls -R /web/
RUN patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 /web/server

FROM debian:bookworm-slim
WORKDIR /web
COPY --from=0 /web /web
COPY --from=0 /etc/ssl /etc/ssl
ENV IP=0.0.0.0

ENTRYPOINT ["/web/server"]
