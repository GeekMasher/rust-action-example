# We do not want to build the full container in Actions,
# we only want to download the latest pre-built container.
FROM ghcr.io/GeekMasher/rust-action-example:main

CMD ["/usr/bin/actions"]

