tag = ogp

dev:
	systemfd --no-pid -s http::8080 -- cargo watch -q -x run

fmt:
	cargo +nightly fmt
	cargo fix --allow-dirty --allow-staged

lint:
	cargo +nightly fmt --check
	cargo check --release --locked

test:
	cargo test

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

docker-clean:
	docker rmi --force $(shell docker images -f "dangling=true" -q)

bench:
	wrk -t4 -c500 -d30s 'http://localhost:8080/health'
	wrk -t4 -c500 -d30s 'http://localhost:8080/v0/svg?title=&author=&photo=http://localhost:8080/assets/favicon.svg&url=&theme=default'
	wrk -t4 -c500 -d30s 'http://localhost:8080/v0/png?title=&author=&photo=http://localhost:8080/assets/favicon.svg&url=&theme=default'
