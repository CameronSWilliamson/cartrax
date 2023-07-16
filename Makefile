.PHONY: dev
.PHONY: prod
.PHONY: clean
.PHONY: down
.PHONY: build
.PHONY: pgup
.PHONY: pgdown
.PHONY: dbuild
.PHONY: dclean

down:
	@docker-compose down --remove-orphans

dev: down
	@docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build

prod: down
	@docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d

clean:
	@rm -rf ./server/target/ ./client/dist/ ./client/node_modules/

.SILENT:
build:
	cd ./client/; npm install; npm run build
	cd ./server/; cargo build

.SILENT:
dbuild:
	docker build --no-cache --pull -t cartrax-client client/
	docker build --no-cache --pull -t cartrax-server client/

.SILENT:
dclean:
	docker image rm -f cartrax-client
	docker image rm -f cartrax-server

pgup:
	@docker run \
		--name cartrax-postgres \
		--hostname cartrax-postgres \
		-e POSTGRES_PASSWORD=password \
		-dp 5432:5423 \
		postgres:latest
	@docker run \
		--name cartrax-adminer \
		--hostname cartrax-adminer \
		--link cartrax-postgres \
		-e ADMINER_DEFAULT_SERVER=cartrax-postgres \
		-dp 8081:8080 \
		adminer:latest 

pgdown:
	@docker rm -f cartrax-postgres cartrax-adminer

.SILENT:
help:
	echo "Makefile for cartrax:"
	echo "    down: Stop existing docker-compose"
	echo "    dev: Start up new development docker-compose"
	echo "    prod: Start up a new production docker-compose"
	echo "    clean: Cleans all build directories"
	echo "    build: Builds client and server"
	echo "    dbuild: Builds client and server docker containers"
	echo "    dclean: Removes client and server docker containers"
	echo "    pgup: Starts a standalone PostgreSQL instance and an adminer instance at http://localhost:8080"
	echo "    pgdown: Tears down PostgreSQL adminer instance"

