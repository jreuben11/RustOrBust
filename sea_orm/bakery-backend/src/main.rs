#![allow(unused)]

mod migrator;
use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::SchemaManager;

mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

async fn get_db_backend(db: DatabaseConnection) -> Result<DatabaseConnection, DbErr> {
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
    let mut la_chef_names: Vec<String> = chefs[0].to_owned().into_iter().map(|b| b.name).collect();
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

async fn run() -> Result<(), DbErr> {
    let mut db = Database::connect(DATABASE_URL).await?;
    db = get_db_backend(db).await?;
    // Migrator::refresh(db).await?;

    check_schema(&db).await?;

    // delete all
    let res: DeleteResult = chef::Entity::delete_many().exec(&db).await?;
    let res: DeleteResult = bakery::Entity::delete_many().exec(&db).await?;

    let chef_id = insert_and_update(&db).await?;
    let bakery_id = find(&db).await?;
    delete(&db, chef_id, bakery_id).await?;
    relational_select(&db).await?;
    load_many(&db).await?;

    mock().await?;
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
