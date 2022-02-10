FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y openssl

COPY example_server example_server

CMD ["sh", "-c", "./example_server"]