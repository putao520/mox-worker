FROM alpine:latest
RUN apk upgrade --update-cache --available && \
    apk add openssl && \
    rm -rf /var/cache/apk/*
WORKDIR /home/app/
ENTRYPOINT ["./worker"]