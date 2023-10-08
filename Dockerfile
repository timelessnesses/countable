FROM python:3.10-alpine
WORKDIR /bot
COPY . .
RUN apk update && apk add make gcc g++ build-base linux-headers git && python3 -m ensurepip && python3 -m pip install poetry && poetry install -vvv && apk del gcc g++ build-base linux-headers
ENV JISHAKU_HIDE=1
ENV I_AM_CONTAINERIZED=YES
ARG REVISION
CMD make
