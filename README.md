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

But when it reached ended sequel however you add a character to it like you counting a number like this

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

There's 2 types you can host this bot.

- Replit (likely the easiest)
- Docker (easier than replit and can be scaled)
- Others (applies to others that they don't show the files publicly)

## Replit

Clone my [repl](https://replit.com/@Mooping/alphabet-count-bot) is the easiest way but you can follow stuff below.  
Clone my repository and tell replit to run the following command everytime repl starts up

```bash
python bot.py
```

You also likely need to install packages using poetry (I think it is install for it automatically)

```bash
poetry install
```

You need to also need to add env var via `Secrets` like:

```dotenv
ALPHABET_DB_HOST=<postgresql server hostname>
ALPHABET_DB_PORT=<postgresql server port>
ALPHABET_DB_USER=<postgresql server username>
ALPHABET_DB_PASSWORD=<postgresql server password>
ALPHABET_DB_NAME=<postgresql database name>
ALPHABET_URI=<postgresql uri if you don't want to type stuff up there>
ALPHABET_TOKEN=<bot token>
IS_REPLIT=any value (this is for replit)
```

Start repl up and you should be good to go(?)

## Docker

Pull it
```bash
docker pull ghcr.io/timelessnesses/countable:main
```
then start it with
```
docker run ghcr.io/timelessnesses/countable -e ALPHABET_TOKEN=token
```

## Others

Rename .env.example to .env and add your env vars.

```dotenv
ALPHABET_DB_HOST=<postgresql server hostname>
ALPHABET_DB_PORT=<postgresql server port>
ALPHABET_DB_USER=<postgresql server username>
ALPHABET_DB_PASSWORD=<postgresql server password>
ALPHABET_DB_NAME=<postgresql database name>
ALPHABET_URI=<postgresql uri if you don't want to type stuff up there>
ALPHABET_TOKEN=<bot token>
```

and run `python -m pip install poetry` then `python -m poetry install` then `python -m poetry shell` and run `python bot.py`

# Current State

- [x] Stable
- [ ] Broken
