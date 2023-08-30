FROM ubuntu:latest

ARG IGGY_CLI_PATH
RUN test -n "$IGGY_CLI_PATH" || (echo "IGGY_CLI_PATH  not set" && false)

ARG IGGY_SERVER_PATH
RUN test -n "$IGGY_SERVER_PATH" || (echo "IGGY_SERVER_PATH  not set" && false)

WORKDIR /iggy

COPY configs ./configs
COPY ${IGGY_CLI_PATH} ./
COPY ${IGGY_SERVER_PATH} ./

CMD ["/iggy/iggy-server"]
