FROM alpine
WORKDIR /bot
COPY . .
RUN apk update && apk add postgresql python3-dev make gcc g++ build-base linux-headers
RUN python3 -m ensurepip
RUN python3 -m pip install poetry
RUN poetry install
ENV ALPHABET_DB_HOST=postgres
ENV ALPHABET_DB_PORT=5432
ENV ALPHABET_DB_USER=postgres
ENV ALPHABET_DB_PASSWORD=postgres
ENV ALPHABET_DB_NAME=postgres
ENV JISHAKU_HIDE=1
ENV ALPHABET_TOKEN=none
CMD make
