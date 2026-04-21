
.PHONY: all up down build compile clean fclean bash re

all: build up compile

up:
	docker compose up -d

down:
	docker compose down

build:
	docker compose build

compile:
	docker compose exec stockholm cargo build --release

clean:
	docker compose down --rmi all --volumes --remove-orphans
	docker compose rm -f

fclean: clean
	rm -rf srcs/target

bash:
	docker compose exec stockholm bash

re: clean up

re: clean all