# ----------------------------------------------------------------------------------------------------------------------
#
#     ____             __             _____ __   
#    / __ \____  _____/ /_____  _____/ __(_) /__ 
#   / / / / __ \/ ___/ //_/ _ \/ ___/ /_/ / / _ \
#  / /_/ / /_/ / /__/ ,< /  __/ /  / __/ / /  __/
# /_____/\____/\___/_/|_|\___/_/  /_/ /_/_/\___/                                           
#
#
# For official documentation, see:
# https://docs.docker.com/engine/reference/builder/
#
# ----------------------------------------------------------------------------------------------------------------------

# ----------------------------------------------------------------------------------------------------------------------
# BUILD ENVIRONMENT
#
# Description:
# Set up environment to create production ready Rust application
#
# For official documentation, see:
# https://docs.docker.com/develop/develop-images/multistage-build
#
# ----------------------------------------------------------------------------------------------------------------------

# Set base image
# See official available Rust base images at Docker Hub https://hub.docker.com/_/rust
FROM rust:1.67.1

# Set current working directory
# See official /opt directory documentation at Linux Filesystem Hierarchy
# https://tldp.org/LDP/Linux-Filesystem-Hierarchy/html/opt.html
WORKDIR /opt/rusty-server

# Copy everything from root path to WORKDIR
COPY . .

# Create production ready binary
RUN cargo build --release

# ----------------------------------------------------------------------------------------------------------------------
# SERVER ENVIRONMENT
#
# Description:
# Set up environment to run production ready Rust
#
# For official documentation, see:
# https://docs.docker.com/develop/develop-images/multistage-build
#
# ----------------------------------------------------------------------------------------------------------------------

# See official available distroless images at Google Cloud https://console.cloud.google.com/gcr/images/distroless
FROM gcr.io/distroless/cc-debian11

# Set arguments which can be later overwritten higher in hierarchy with ex. Docker Compose, or .gitlab-ci.yml
ARG VERSION="0.0.1"
ARG PORT=7878
ARG DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres?schema=public"
ARG SUPERTOKENS_URL=""
ARG SUPERTOKENS_API_KEYS=""

# Set environmental variables from arguments
ENV VERSION=$VERSION
ENV PORT=$PORT
ENV DATABASE_URL=$DATABASE_URL
ENV SUPERTOKENS_URL=$SUPERTOKENS_URL
ENV SUPERTOKENS_API_KEYS=$SUPERTOKENS_API_KEYS

# Copy binary from build environment stage to current root path
COPY --from=0 /opt/rusty-server/target/release/rusty_server .

# Run the binary
CMD ["./rusty_server"]

# Run container healthcheck to see that the Rust server responds
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
  CMD curl -f http://locahlhost:7878 || exit 1
