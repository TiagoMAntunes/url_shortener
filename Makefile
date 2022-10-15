up: build
	docker-compose up -d
	
build:
	docker-compose build

watch: build
	docker-compose up

down:
	docker-compose down

test: build
	docker-compose up -d
	cargo test
	docker-compose down