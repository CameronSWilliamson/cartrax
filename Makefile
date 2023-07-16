.PHONY: dev
dev: down
	docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build

.PHONY: prod
prod: down
	docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d

.PHONY: down
down:
	docker-compose down --remove-orphans

.PHONY: clean
clean:
	rm -rf ./server/target/ ./client/dist/ ./client/node_modules/

.PHONY: build
build:
	cd ./client/; npm install; npm run build; cd ../server/; cargo build

