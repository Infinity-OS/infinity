# Use Ubuntu as a base image
FROM ubuntu:16.04

# Install the following tools:
#   - build-essential
#   - grup-mkrescue
#   - nasm
#   - xorriso
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    nasm \
    xorriso

# Install Xargo
COPY xargo.sh /
RUN bash /xargo.sh

# Install LLVM.LLD
COPY lld.sh /
RUN bash /lld.sh
