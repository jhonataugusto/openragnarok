# rAthena build & runtime image (Alpine).
# - Compiles the server binaries on first run via entrypoint.sh
# - Source tree is bind-mounted from the host at /rathena
FROM alpine:3.20

WORKDIR /rathena

RUN apk add --no-cache \
        bash \
        ca-certificates \
        cmake \
        curl \
        g++ \
        gcc \
        git \
        linux-headers \
        make \
        mariadb-client \
        mariadb-connector-c-dev \
        mariadb-dev \
        netcat-openbsd \
        pcre-dev \
        perl \
        zlib-dev \
    && curl -fsSL https://raw.githubusercontent.com/eficode/wait-for/v2.2.4/wait-for \
        -o /usr/local/bin/wait-for \
    && chmod +x /usr/local/bin/wait-for

COPY entrypoint.sh /usr/local/bin/entrypoint.sh

# Strip any CRLF that may sneak in from a Windows host so /bin/bash can run it
RUN sed -i 's/\r$//' /usr/local/bin/entrypoint.sh \
    && chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["help"]
