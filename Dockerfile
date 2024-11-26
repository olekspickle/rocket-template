# Run detached and remove container when it stopped
# docker build -t rocket-template .
# docker run -dp 3000:3000 --rm --name server rocket-template

FROM rust:1.66 as build

# create a new empty shell project
RUN USER=root cargo new --bin rocket-template
WORKDIR /rocket-template

# copy over your manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
COPY Rocket.toml ./Rocket.toml


# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY src src
# copy static files like templates and styles
COPY static static

# build for release
RUN cargo build --release

# our final slim base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /rocket-template/target/release/rocket-template .
COPY --from=build /rocket-template/static static
COPY --from=build /rocket-template/Rocket.toml .
EXPOSE 8000

# set the startup command to run your binary
CMD ["./rocket-template"]
