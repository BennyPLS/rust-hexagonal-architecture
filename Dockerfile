# Build Container
FROM rust:1.79 as build

# 1. Create a new empty shell project
RUN USER=root cargo new hexagonal
WORKDIR /hexagonal

# 2. Copy our workspace Cargo flies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 2.1. Copy our contexts Cargo flies
COPY ./contexts/Cargo.toml ./contexts/Cargo.toml
COPY ./contexts/Cargo.toml ./contexts/Cargo.toml

# 2.2. Copy our apps Cargo flies
COPY ./apps/Cargo.toml ./apps/Cargo.toml
COPY ./apps/Cargo.toml ./apps/Cargo.toml

# 2.3. Copy & Move some dummy main files to deps (apps & contexts)
RUN cp ./src ./contexts/ -r
RUN mv ./src ./apps/

# 3. Build dependecies
RUN cargo build --release

# 4. Copy Source Code
COPY ./apps ./apps
COPY ./contexts ./contexts

# 5. Build
RUN cargo build --release

# Executable Container
FROM rust:1.79

# 6. Copy exectuable file
COPY --from=build /hexagonal/target/release/ .
COPY Rocket.toml .

# 7. Execute app
CMD ["./apps"]
