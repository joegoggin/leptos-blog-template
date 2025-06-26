read -r -p "Enter your pgadmin email: " EMAIL
read -r -sp "Enter your postgres password: " PASSWORD
echo

if [ ! -f .env ]; then
	echo "DATABASE_URL=\"postgresql://postgres:$PASSWORD@localhost:5432/leptos-blog-template\"" >>.env
fi

if [ ! -d ./docker/env ]; then
	cd docker && mkdir env
fi

if [ ! -f ./docker/env/postgres.env ]; then
	touch docker/env/postgres.env
	echo "POSTGRES_USER=\"postgres\"" >>./docker/env/postgres.env
	echo "POSTGRES_PASSWORD=\"$PASSWORD\"" >>./docker/env/postgres.env
fi

if [ ! -f ./env/env/pgadmin.env ]; then
	touch docker/env/pgadmin.env
	echo "PGADMIN_DEFAULT_EMAIL=\"$EMAIL\"" >>./docker/env/pgadmin.env
	echo "PGADMIN_DEFAULT_PASSWORD=\"$PASSWORD\"" >>./docker/env/pgadmin.env
	echo "PGADMIN_CONFIG_SERVER_MODE=\"False\"" >>./docker/env/pgadmin.env
fi
