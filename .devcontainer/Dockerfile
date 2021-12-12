# syntax=docker/dockerfile:1.3-labs
FROM rust:1.57-slim-bullseye

ENV \
  TZ=Asia/Tokyo \
  LANG=ja_JP.UTF-8 \
  INSTALL_PKG="env DEBIAN_FRONTEND=noninteractive apt-get -qq --no-install-recommends --yes install"

RUN apt-get -qq update
RUN <<EOF
  $INSTALL_PKG fish jq ncdu git vim file
  chsh --shell /usr/bin/fish
EOF

RUN <<EOF
  $INSTALL_PKG mingw-w64
  rustup target add x86_64-pc-windows-gnu
EOF

RUN rustup component add rust-analysis rust-src rls rustfmt clippy
