# countable

A alphabet count bot inspired by Counting

## How this works

You continue saying alphabet in order like

```text
a
b
... 
z
```

But when the sequence is ended you should add another alphabet like this

```text
aa
ab
...
az
ba
bb
bc
...
bz
```

etc.  
If you ruined the chain you will be logged and people can look at who ruined the chain.  
This entire came from snaky's idea and inspired by counting bot

## How to setup

### Compose (Easiest)

Create new file called `compose.yaml` with these contents inside:

```yaml
services:
    countable:
        build:
            image: ghcr.io/timelessnesses/countable
        environment:
            - ALPHABET_DB_HOST=db
            - ALPHABET_DB_PORT=5432
            - ALPHABET_DB_USER=countable
            - ALPHABET_DB_PASSWORD=countable
            - ALPHABET_DB_NAME=countables
            - ALPHABET_TOKEN=<TOKEN>
    db:
        image: postgres:14
        environment:
            - POSTGRES_USER=countable
            - POSTGRES_PASSWORD=countable
            - POSTGRES_DB=countables
```

Then start it up with `docker compose up` (add `-d` for running it in the background)

### Running directly from an image

Clone the image with `docker pull ghcr.io/timelessnesses/countable:latest` and run it by

```sh
docker run ghcr.io/timelessnesses/countable -e ALPHABET_DB_HOST=<Database's hostname> -e ALPHABET_DB_PORT=<Database's server port> -e ALPHABET_DB_USER=<User account for accessing database's server> -e ALPHABET_DB_PASSWORD=<A password for user account> -e ALPHABET_DB_NAME=<Database name> -e ALPHABET_TOKEN=<Discord bot token>
```

### Cloned from GitHub Repository (Compose)

Create `.env` with these contents inside:

```sh
ALPHABET_TOKEN=<TOKEN>
```

Then run `docker compose up` (add `-d` for running it in the background

### Cloned from GitHub Repository (Image)

Run `docker build . --tag countable_dev:latest` then run

```sh
docker run countable_dev:latest -e ALPHABET_DB_HOST=<Database's hostname> -e ALPHABET_DB_PORT=<Database's server port> -e ALPHABET_DB_USER=<User account for accessing database's server> -e ALPHABET_DB_PASSWORD=<A password for user account> -e ALPHABET_DB_NAME=<Database name> -e ALPHABET_TOKEN=<Discord bot token>
```

# Current State

- [x] Stable
- [ ] Broken
