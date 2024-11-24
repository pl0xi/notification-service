#! /bin/bash
set -e

docker compose -f docker-compose.test.yml up -d

until docker-compose -f docker-compose.test.yml exec -T test-db pg_isready --quiet; do
    echo -ne "\r⌛ Waiting for test database to be ready..."
    sleep 0.5
    echo -ne "\r⏳ Waiting for test database to be ready..."
    sleep 0.5
done

cargo test --test '*' --verbose  -- --nocapture

docker compose -f docker-compose.test.yml down