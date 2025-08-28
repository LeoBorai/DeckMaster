FROM alpine:3

COPY ./tmp/deckmaster-server /opt/deckmaster-server

COPY ./tmp/mtg.sqlite /data/mtg.sqlite

ENTRYPOINT ["/opt/deckmaster-server"]
