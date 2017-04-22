# Prepare the environment to build, using our Docker image
# 	- Build the Docker image that contains the toolchain
# 	- Run the configure script
d_init:
	@docker build -t infinity .
	@docker run --rm -v "${PWD}:/code" infinity sh configure.sh

# Build Infinity OS using the Docker toolchain
d_make:
	@docker run -t --rm -v "${PWD}:/code" infinity make

# Starts an interactive session
d_inter:
	@docker run --rm -it --entrypoint /bin/bash -v "${PWD}:/code" infinity
