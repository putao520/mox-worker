FROM alpine:3.19
RUN apk upgrade --update-cache --available && \
    apk add openssl musl-dev gcc pkgconfig openssl-dev && \
    rm -rf /var/cache/apk/*
WORKDIR /home/app/
ENTRYPOINT ["./worker"]