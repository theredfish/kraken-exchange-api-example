FROM rust:1.50

ENV APP_USER=rust
ENV APP_DIR=/usr/stonk

RUN groupadd ${APP_USER} \
    && useradd -g ${APP_USER} ${APP_USER} \
    && mkdir -p ${APP_DIR}

COPY . ${APP_DIR}
RUN chown -R ${APP_USER}:${APP_USER} ${APP_DIR}
USER ${APP_USER}
WORKDIR ${APP_DIR}

# First build step for caching dependencies in this layer. We build the tests
# without launching them.
RUN cargo test --no-run

# Run the integration tests with cucumber. Since we do not have unit / doc tests
# it doesn't matter if we do not specify the test name. This is a default
# command that can be easily override.
CMD ["/usr/local/cargo/bin/cargo", "test"]
