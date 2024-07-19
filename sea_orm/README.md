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
sea-orm = {version="0.12.15" , features = [ "sqlx-mysql", "runtime-async-std-native-tls", "macros", "mock" ]}
sea-orm-migration = "0.12.15"
rocket = {version="0.5.1", features = ["json"]}
async-graphql = "7.0.7"
async-graphql-rocket = "7.0.7"
```

## [migrations](bakery-backend/migration/src/lib.rs)
```rust
pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20240716_000001_create_bakery_table;
mod m20240716_000002_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20240716_000001_create_bakery_table::Migration),
            Box::new(m20240716_000002_create_chef_table::Migration),
        ]
    }
}
```

## [main.rs](bakery-backend/src/main.rs)
```rust
mod migrator;
use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::SchemaManager;
mod entities;
use entities::{prelude::*, *};


async fn run() -> Result<(), DbErr> {
    let mut db = Database::connect(DATABASE_URL).await?;
    db = get_db_backend(db).await?;
    // Migrator::refresh(db).await?;
    check_schema(&db).await?;
    delete_all(&db).await?;
    let chef_id = insert_and_update(&db).await?;
    let bakery_id = find(&db).await?;
    delete(&db, chef_id, bakery_id).await?;
    relational_select(&db).await?;
    build_sql(&db).await?;
    load_many(&db).await?;
    mock().await?;
    Ok(())
}
```

## Rocket REST API
```rust
use rocket::*;
use rocket::serde::json::Json;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder { ... }
impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder { ... }
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> { ... }

async fn rocket() -> _ {
    if let Err(err) = block_on(database_access::init_data()) {
        panic!("{}", err);
    }
    // TODO: - DatabaseConnection is not Clone, Sync, Send
    let db1 = match database_access::get_db_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    let db2 = match database_access::get_db_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
   
   let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
       .data(db2) // Add the database connection to the GraphQL global context
       .finish();

    rocket::build()
        .manage(db1)
        .manage(schema) // GraphQL
        .mount("/", routes![index, bakeries, graphql_request, graphiql])
}
```



## Rocket GraphQL API
```rust

```