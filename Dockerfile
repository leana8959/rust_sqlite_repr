FROM rust:1.70
RUN apt-get update && apt-get install -y sqlite3
WORKDIR /usr/src/myapp/
COPY . .
