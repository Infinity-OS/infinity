FROM base/archlinux

RUN echo "[archlinuxfr]" >> /etc/pacman.conf && \
    echo "SigLevel = Never" >> /etc/pacman.conf && \
    echo "Server = http://repo.archlinux.fr/x86_64" >> /etc/pacman.conf &&\
    pacman -Sy

# Install Yaourt
RUN pacman --sync --noconfirm --noprogressbar --quiet sudo yaourt

# Install the following tools:
#   - build-essential
#   - grup-mkrescue
#   - nasm
#   - xorriso
RUN yaourt --noconfirm -Sa \
    gcc make rustup \
    autoconf autoconf automake libtool \
    base-devel \
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

RUN source ~/.cargo/env

# Add Rust source components
RUN rustup component add rust-src

RUN ~/.cargo/bin/cargo-install-update || cargo install cargo-update
RUN ~/.cargo/bin/rustfmt || cargo install rustfmt

# Install Xargo and force update
RUN cargo install xargo
RUN cargo install-update -a

# Define a volume and set the working directory
VOLUME ["/code"]
WORKDIR /code
