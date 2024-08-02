# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/pagerank

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the dependencies
RUN cargo build --release


# Build the project
RUN cargo install --path .

# Specify the command to run the application
CMD ["pagerank"]