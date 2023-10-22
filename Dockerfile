FROM rust:1.73.0-alpine3.18 as build

WORKDIR /usr/src/app
RUN apk add --update --no-cache \
    --repository https://dl-cdn.alpinelinux.org/alpine/v3.18/community \
    --repository https://dl-cdn.alpinelinux.org/alpine/v3.18/main \
    musl-dev \
    vips-dev

COPY . .

RUN RUSTFLAGS="-C target-feature=-crt-static $(pkg-config vips --libs)" cargo build --release

FROM alpine:3.18

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    ROCKET_ADDRESS=0.0.0.0

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk add --update --no-cache \
    --repository https://dl-cdn.alpinelinux.org/alpine/v3.18/community \
    --repository https://dl-cdn.alpinelinux.org/alpine/v3.18/main \
    vips

COPY --from=build /usr/src/app/templates ${APP}/templates
COPY --from=build /usr/src/app/target/release/htmx-demo ${APP}/htmx-demo

RUN mkdir -p ${APP}/upload

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./htmx-demo"]