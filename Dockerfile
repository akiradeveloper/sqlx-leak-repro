FROM 'rust:1.55.0'
RUN rustup component add rustfmt

RUN wget https://github.com/k0kubun/sqldef/releases/download/v0.11.10/sqlite3def_linux_amd64.tar.gz \
    && tar xvfz sqlite3def_linux_amd64.tar.gz \
	&& mv sqlite3def /usr/local/bin

RUN apt-get update \
	&& apt-get install -y sqlite3

COPY . .
ENTRYPOINT sh run-inner.sh