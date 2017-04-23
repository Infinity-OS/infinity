set -ex

main() {
    local dependencies=(
        ca-certificates
        curl
    )

    sudo apt-get update
    local purge_list=()
    for dep in ${dependencies[@]}; do
        if ! dpkg -L $dep; then
            sudo apt-get install --no-install-recommends -y $dep
            purge_list+=( $dep )
        fi
    done

    cat <<EOF >>/etc/apt/sources.list
deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-4.0 main
deb-src http://apt.llvm.org/xenial/ llvm-toolchain-xenial-4.0 main
EOF

    curl -L http://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
    sudo apt-get update
    sudo apt-get install --no-install-recommends -y lld-4.0
    ln -s ld.lld-4.0 /usr/bin/ld.lld

    # Clean up
    sudo apt-get purge --auto-remove -y ${purge_list[@]}

    rm $0
}

main "${@}"
