tag = ogp

dev:
	cargo watch -q -x 'run'

lint:
	cargo fmt --check
	cargo check --release --locked

update:
	cargo upgrade -i

deploy:
	fly deploy

docker-build:
	docker build -t $(tag) .
	docker images -q $(tag) | xargs docker inspect -f '{{.Size}}' | xargs numfmt --to=iec

docker-run: docker-build
	docker rm --force $(tag) || true
	docker run -p 8080:8080 --env-file .env --name $(tag) $(tag)
