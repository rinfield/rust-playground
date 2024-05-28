# syntax=docker/dockerfile:1.3-labs
FROM rust:1-slim-bookworm

ENV \
  TZ=Asia/Tokyo \
  LANG=C \
  INSTALL_PKG="env DEBIAN_FRONTEND=noninteractive apt-get -qq --no-install-recommends --yes install"

RUN apt-get -qq update
RUN <<EOF
  $INSTALL_PKG fish jq ncdu git vim file
  chsh --shell /usr/bin/fish
EOF

#RUN rustup component add rust-analysis rust-src rls rustfmt clippy
