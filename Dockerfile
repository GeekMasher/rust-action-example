# We do not want to build the full container in Actions,
# we only want to download the latest pre-built container.
FROM ghcr.io/Mathew Payne/example-action:main

CMD ["/usr/bin/actions"]

