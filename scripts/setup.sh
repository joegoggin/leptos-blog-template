source ./scripts/generate_env.sh

docker compose up -d

sleep 1

docker exec -e PGPASSWORD="$PASSWORD" leptos-blog-template_postgres psql -U postgres -d postgres -f "/tmp/init.sql"

sqlx migrate run
