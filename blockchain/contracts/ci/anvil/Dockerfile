# Use the latest foundry image
FROM ghcr.io/lit-protocol/foundry

COPY ./start_anvil.sh ./start_anvil.sh
RUN chmod +x ./start_anvil.sh

EXPOSE 8545

# Define service as up when anvil is listening.
HEALTHCHECK --interval=5s --timeout=3s \
    CMD grep 'Listening on 0.0.0.0:8545' anvil.log

# Start the anvil server with 10 accounts
ENTRYPOINT ["./start_anvil.sh"]