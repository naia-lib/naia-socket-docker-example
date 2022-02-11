FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y openssl

COPY server_example server_example

CMD ["sh", "-c", "./server_example"]