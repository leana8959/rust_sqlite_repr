FROM rust:1.70

WORKDIR /tmp/
RUN wget https://www.sqlite.org/2023/sqlite-autoconf-3420000.tar.gz && \
    tar xfz sqlite-autoconf-3420000.tar.gz
WORKDIR /tmp/sqlite-autoconf-3420000
RUN ./configure && make && make install
RUN [ -n $(which sqlite3) ] # assert installation

WORKDIR /usr/src/myapp/
COPY . .
