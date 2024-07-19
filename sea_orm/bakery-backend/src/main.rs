#![allow(unused)]

mod migrator;
use futures::executor::block_on;
mod entities;
use sea_orm::*;
use sea_orm_migration::SchemaManager;
use entities::{prelude::*, *};
mod database_access {

    use super::*;

    const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
    const DB_NAME: &str = "bakeries_db";

    pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
        let mut db = Database::connect(DATABASE_URL).await?;
        let db = match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
                ))
                .await?;

                let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
                ))
                .await?;
                db.execute(Statement::from_string(
                    db.get_database_backend(),
                    format!("CREATE DATABASE \"{}\";", DB_NAME),
                ))
                .await?;

                let url = format!("{}/{}", DATABASE_URL, DB_NAME);
                Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        };
        return Ok(db);
    }

    async fn delete_all(db: &DatabaseConnection) -> Result<(), DbErr> {
        let res: DeleteResult = chef::Entity::delete_many().exec(db).await?;
        let res: DeleteResult = bakery::Entity::delete_many().exec(db).await?;
        Ok(())
    }

    async fn check_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
        let schema_manager = SchemaManager::new(db); // To investigate the schema

        assert!(schema_manager.has_table("bakery").await?);
        assert!(schema_manager.has_table("chef").await?);
        Ok(())
    }

    async fn insert_and_update(db: &DatabaseConnection) -> Result<i32, DbErr> {
        // Insert a bakery
        let happy_bakery = bakery::ActiveModel {
            name: ActiveValue::Set("Happy Bakery".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let res = Bakery::insert(happy_bakery).exec(db).await?;

        // Update the bakery
        let sad_bakery = bakery::ActiveModel {
            id: ActiveValue::Set(res.last_insert_id),
            name: ActiveValue::Set("Sad Bakery".to_owned()),
            profit_margin: ActiveValue::NotSet,
        };
        sad_bakery.update(db).await?;

        // Insert a chef
        let john = chef::ActiveModel {
            name: ActiveValue::Set("John".to_owned()),
            bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
            ..Default::default()
        };
        let res = Chef::insert(john).exec(db).await?;
        let chef_id = res.last_insert_id;
        Ok(chef_id)
    }

    async fn find(db: &DatabaseConnection) -> Result<i32, DbErr> {
        // Finding all is built-in
        let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert_eq!(bakeries.len(), 1);
        let bakery_id = bakeries[0].id;

        // Finding by id is built-in
        let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(bakery_id).one(db).await?;
        assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

        // Finding by arbitrary column with `filter()`
        let sad_bakery: Option<bakery::Model> = Bakery::find()
            .filter(bakery::Column::Name.eq("Sad Bakery"))
            .one(db)
            .await?;
        assert_eq!(sad_bakery.unwrap().id, bakery_id);
        Ok(bakery_id)
    }

    async fn delete(db: &DatabaseConnection, chef_id: i32, bakery_id: i32) -> Result<(), DbErr> {
        // Delete
        let john = chef::ActiveModel {
            id: ActiveValue::Set(chef_id), // The primary key must be set
            ..Default::default()
        };
        john.delete(db).await?;

        let sad_bakery = bakery::ActiveModel {
            id: ActiveValue::Set(bakery_id), // The primary key must be set
            ..Default::default()
        };
        sad_bakery.delete(db).await?;

        let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert!(bakeries.is_empty());
        Ok(())
    }

    async fn relational_select(db: &DatabaseConnection) -> Result<(), DbErr> {
        // relational select
        let la_boulangerie = bakery::ActiveModel {
            name: ActiveValue::Set("La Boulangerie".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;

        for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }

        let la_boulangerie: bakery::Model = Bakery::find_by_id(bakery_res.last_insert_id)
            .one(db)
            .await?
            .unwrap();

        let chefs: Vec<chef::Model> = la_boulangerie.find_related(Chef).all(db).await?;
        let mut chef_names: Vec<String> = chefs.into_iter().map(|b| b.name).collect();
        chef_names.sort_unstable();

        assert_eq!(chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
        Ok(())
    }

    async fn load_many(db: &DatabaseConnection) -> Result<(), DbErr> {
        // loader
        // Inserting two bakeries and their chefs
        let la_boulangerie = bakery::ActiveModel {
            name: ActiveValue::Set("La Boulangerie".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
        for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }
        let la_id = bakery_res.last_insert_id;

        let arte_by_padaria = bakery::ActiveModel {
            name: ActiveValue::Set("Arte by Padaria".to_owned()),
            profit_margin: ActiveValue::Set(0.2),
            ..Default::default()
        };
        let bakery_res = Bakery::insert(arte_by_padaria).exec(db).await?;
        for chef_name in ["Brian", "Charles", "Kate", "Samantha"] {
            let chef = chef::ActiveModel {
                name: ActiveValue::Set(chef_name.to_owned()),
                bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
                ..Default::default()
            };
            Chef::insert(chef).exec(db).await?;
        }
        let arte_id = bakery_res.last_insert_id;

        // First find bakeries as Models
        let bakeries: Vec<bakery::Model> = Bakery::find()
            .filter(
                Condition::any()
                    .add(bakery::Column::Id.eq(la_id))
                    .add(bakery::Column::Id.eq(arte_id)),
            )
            .all(db)
            .await?;

        // Then use loader to load the chefs in one query.
        let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;
        let mut la_chef_names: Vec<String> =
            chefs[0].to_owned().into_iter().map(|b| b.name).collect();
        la_chef_names.sort_unstable();
        let mut arte_chef_names: Vec<String> =
            chefs[1].to_owned().into_iter().map(|b| b.name).collect();
        arte_chef_names.sort_unstable();

        assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);
        assert_eq!(arte_chef_names, ["Brian", "Charles", "Kate", "Samantha"]);
        Ok(())
    }

    async fn mock() -> Result<(), DbErr> {
        let db = &MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results([
                // First query result
                vec![bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                }],
                // Second query result
                vec![
                    bakery::Model {
                        id: 1,
                        name: "Happy Bakery".to_owned(),
                        profit_margin: 0.0,
                    },
                    bakery::Model {
                        id: 2,
                        name: "Sad Bakery".to_owned(),
                        profit_margin: 100.0,
                    },
                    bakery::Model {
                        id: 3,
                        name: "La Boulangerie".to_owned(),
                        profit_margin: 17.89,
                    },
                ],
            ])
            .append_query_results([
                // Third query result
                vec![
                    chef::Model {
                        id: 1,
                        name: "Jolie".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
                        id: 2,
                        name: "Charles".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
                        id: 3,
                        name: "Madeleine".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                    chef::Model {
                        id: 4,
                        name: "Frederic".to_owned(),
                        contact_details: None,
                        bakery_id: 3,
                    },
                ],
            ])
            .into_connection();

        let happy_bakery: Option<bakery::Model> = Bakery::find().one(db).await?;
        assert_eq!(
            happy_bakery.unwrap(),
            bakery::Model {
                id: 1,
                name: "Happy Bakery".to_owned(),
                profit_margin: 0.0,
            }
        );

        let all_bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
        assert_eq!(
            all_bakeries,
            vec![
                bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                },
                bakery::Model {
                    id: 2,
                    name: "Sad Bakery".to_owned(),
                    profit_margin: 100.0,
                },
                bakery::Model {
                    id: 3,
                    name: "La Boulangerie".to_owned(),
                    profit_margin: 17.89,
                },
            ]
        );

        let la_boulangerie_chefs: Vec<chef::Model> = Chef::find().all(db).await?;
        assert_eq!(
            la_boulangerie_chefs,
            vec![
                chef::Model {
                    id: 1,
                    name: "Jolie".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 2,
                    name: "Charles".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 3,
                    name: "Madeleine".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
                chef::Model {
                    id: 4,
                    name: "Frederic".to_owned(),
                    contact_details: None,
                    bakery_id: 3,
                },
            ]
        );
        Ok(())
    }

    use sea_query::{Alias, Expr, JoinType, MysqlQueryBuilder, Order, Query};
    #[derive(FromQueryResult)]
    struct ChefNameResult {
        name: String,
    }
    async fn build_sql(db: &DatabaseConnection) -> Result<(), DbErr> {
        use sea_query::{Alias, Query};

        // insert
        let columns: Vec<Alias> = ["name", "profit_margin"]
            .into_iter()
            .map(Alias::new)
            .collect();
        let mut stmt = Query::insert();
        stmt.into_table(bakery::Entity).columns(columns);
        stmt.values_panic(["SQL Bakery".into(), (-100.0).into()]);
        println!("{}", stmt.to_string(MysqlQueryBuilder));
        let builder = db.get_database_backend();
        db.execute(builder.build(&stmt)).await?;

        // select
        let column = (chef::Entity, Alias::new("name"));
        let mut stmt = Query::select();
        stmt.column(column.clone()) // Use `expr_as` instead of `column` if renaming is necessary
            .from(chef::Entity)
            .join(
                JoinType::Join,
                bakery::Entity,
                Expr::col((chef::Entity, Alias::new("bakery_id")))
                    .equals((bakery::Entity, Alias::new("id"))),
            )
            .order_by(column, Order::Asc);
        println!("{}", stmt.to_string(MysqlQueryBuilder));
        let builder = db.get_database_backend();
        let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
            .all(db)
            .await?;
        let chef_names = chef.into_iter().map(|b| b.name).collect::<Vec<_>>();
        assert_eq!(
            chef_names,
            vec!["Charles", "Frederic", "Jolie", "Madeleine"]
        );

        Ok(())
    }

    pub async fn init_data() -> Result<(), DbErr> {
        let db = get_db_connection().await?;
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
}

use rocket::*;
use rocket::serde::json::Json;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

// The following impl's are for easy conversion of error types.

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}

#[get("/test")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[get("/bakeries")]
async fn bakeries(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
         .unwrap()
        // .map_err(Into::into)?
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();

    Ok(Json(bakery_names))
}

#[launch] // The "main" function of the program
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
        .manage(schema) 
        .mount("/", routes![index, bakeries, graphql_request, graphiql])
}

mod schema;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use schema::*;
use rocket::response::content;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/graphiql")]
fn graphiql() -> content::RawHtml<String> {
    content::RawHtml(GraphiQLSource::build().endpoint("/graphql").finish())
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
   request.execute(schema.inner()).await
}
