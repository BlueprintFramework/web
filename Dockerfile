FROM alpine:latest

# Add web and entrypoint
ARG TARGETPLATFORM
COPY .docker/${TARGETPLATFORM#linux/}/web /usr/bin/web

ENTRYPOINT ["/usr/bin/web"]
