# Use Ubuntu as a base image
FROM ubuntu:16.04

# Install the following tools:
#   - build-essential
#   - grup-mkrescue
#   - nasm
#   - xorriso
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    autoconf automake autotools-dev libtool xutils-dev \
    build-essential \
    ca-certificates \
    curl \
    git \
    nasm \
    xorriso

# Install Rustup
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

# Add cargo to PATH
ENV PATH=/root/.cargo/bin:$PATH

# Add Rust source components
RUN rustup component add rust-src

# Install Xargo
COPY utils/xargo.sh /
RUN bash /xargo.sh

# Install LLVM.LLD
COPY utils/lld.sh /
RUN bash /lld.sh

# Define a volume and set the working directory
VOLUME ["/code"]
WORKDIR /code
