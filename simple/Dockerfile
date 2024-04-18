# stage 1 -- build server application
FROM rust:latest AS server
ARG NAME="{{project-name}}"
ARG TARGET="x86_64-unknown-linux-musl"
WORKDIR /app
RUN rustup target add ${TARGET}
COPY src ./src
COPY templates ./templates
COPY Cargo.toml .
RUN cargo test --all-features
RUN cargo build --release --target ${TARGET}
RUN strip target/${TARGET}/release/${NAME}
RUN ls -hla target/${TARGET}/release
RUN cargo --version

# stage 2 -- build content files
FROM node:18 AS content
WORKDIR /app
COPY assets ./assets
COPY templates ./templates
COPY input.css .
COPY package.json .
COPY tailwind.config.js .
RUN npm install 
RUN npm run build-release

# stage 3 -- assemble runtime image
FROM scratch
ARG NAME="{{project-name}}"
ARG CONFIG="website.toml"
ARG TARGET="x86_64-unknown-linux-musl"
WORKDIR /
COPY --from=server  /app/target/${TARGET}/release/${NAME} server
COPY --from=content /app/assets ./assets
COPY ${CONFIG} ./config.toml
USER 1000
CMD ["./server", "--config", "config.toml"]
