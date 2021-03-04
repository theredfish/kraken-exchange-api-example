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

# Cache lib dependencies in this layer
RUN cargo build

# Cache dev dependencies in this layer
RUN cargo test --test bdd --no-run

# Run the integration tests with cucumber.
CMD ["/usr/local/cargo/bin/cargo", "test", "--test", "bdd", "--", "--debug"]
