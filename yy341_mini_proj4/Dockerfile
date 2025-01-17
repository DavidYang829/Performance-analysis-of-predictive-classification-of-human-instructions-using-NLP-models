# Use the official Rust 1.75 image as the base image for the builder stage
FROM rust:1.75 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/week4-mini-project

# Change the user to root for copying files (optional, depending on your needs)
USER root

# Copy the entire content of the local directory into the container's working directory
COPY . .

# Build the project using Cargo with the --release flag for optimized code
RUN cargo build --release

# Expose port 8080 to the outside world (used by Actix-Web)
EXPOSE 8080

# Define the default command to run when the container starts, using 'cargo run'
CMD cargo run
