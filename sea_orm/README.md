# tutorial - bakery backend
https://www.sea-ql.org/sea-orm-tutorial

## setup
```bash
docker run --name mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=password -d mysql:latest 
docker container ps
docker container stop / start mysql

cargo install sea-orm-cli
# List all available migration commands that are supported by `sea-orm-cli`
$ sea-orm-cli migrate -h
# Initialize the migration folder:
$ sea-orm-cli migrate init

# verify
DATABASE_URL="mysql://root:password@localhost:3306/bakeries_db" sea-orm-cli migrate refresh
mysql -u root -p --host 0.0.0.0 --port 3306
use bakeries_db; show tables;

sea-orm-cli generate entity \
    -u mysql://root:password@localhost:3306/bakeries_db \
    -o src/entities
```

## [Cargo.toml](bakery-backend/Cargo.toml)
```toml
[dependencies]
futures = "0.3.30"
sea-orm = {version="0.12.15" , features = [ "sqlx-mysql", "runtime-async-std-native-tls", "macros" ]}
```
