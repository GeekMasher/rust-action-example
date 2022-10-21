# We do not want to build the full container in Actions,
# we only want to download the latest pre-built container.
FROM ghcr.io/geekmasher/rust-action-example:main

# Default Log Level of the Action
ENV RUST_LOG info

# Command in the Action to run
CMD ["/usr/bin/actions"]
