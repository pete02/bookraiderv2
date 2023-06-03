FROM rust:latest
EXPOSE 3000
COPY ./ ./

RUN rm -rf frontend

RUN apt update && apt upgrade

RUN apt install ffmpeg

COPY ./frontend/build ./frontend/build

RUN cargo build --release

RUN rm -rf /src

CMD [ "./target/release/bookraiderv2" ]
