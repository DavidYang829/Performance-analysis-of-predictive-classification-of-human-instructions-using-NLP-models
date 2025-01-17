# Use the latest cargo-lambda environment for building
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder
WORKDIR /usr/src/app
COPY . .
# Build the project for the ARM64 architecture
RUN cargo lambda build --release --arm64

# Use AWS provided base image for ARM64
FROM public.ecr.aws/lambda/provided:al2-arm64
WORKDIR /mini10
# Copy the built executable and dependencies from the builder stage
COPY --from=builder /usr/src/app/target/ ./
# Copy the specific model file for your application
COPY --from=builder /usr/src/app/src/bloom-560m-q5_1-ggjt.bin ./

# Check if the required directory exists
RUN if [ -d /mini10/lambda/yy341_mini_proj10/ ]; then echo "Directory exists"; else echo "Directory does not exist"; fi
# Check if the required executable exists
RUN if [ -f /mini10/lambda/yy341_mini_proj10/bootstrap ]; then echo "File exists"; else echo "File does not exist"; fi

# Set the entrypoint to the bootstrap executable
ENTRYPOINT ["/mini10/lambda/yy341_mini_proj10/bootstrap"]
