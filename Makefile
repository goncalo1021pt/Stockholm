.PHONY: up down build clean

up:
	docker compose up -d

down:
	docker compose down

build:
	docker compose build

clean:
	docker compose down --rmi all --volumes --remove-orphans

bash:
	docker compose exec stockholm bash

re: clean up