FROM rust
WORKDIR /project
COPY ./src .
COPY ./config/.vscode_extensions /root/.vscode-server/.

RUN mkdir /root/.ssh
COPY ./config/github /root/.ssh/.

RUN rustup component add rustfmt
