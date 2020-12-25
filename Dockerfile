FROM rust:1.48.0-slim-buster

ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8 \
    SQLDEF_VERSION=v0.8.6

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     libpq-dev \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  \
  && echo "install sqldef" \
  && curl -L -O https://github.com/k0kubun/sqldef/releases/download/${SQLDEF_VERSION}/psqldef_linux_amd64.tar.gz \
  && tar xf psqldef_linux_amd64.tar.gz \
  && rm psqldef_linux_amd64.tar.gz \
  && mv psqldef /usr/local/bin \
  \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make

WORKDIR /app

CMD ["cargo", "run"]
