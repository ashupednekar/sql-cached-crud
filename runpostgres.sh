docker rm -f postgres
docker run --name postgres -e POSTGRES_USER=user -e POSTGRES_PASSWORD=passwd -p 5432:5432 -d postgres
sleep 2
docker exec -it postgres psql -U user -d postgres -c "create database db"
