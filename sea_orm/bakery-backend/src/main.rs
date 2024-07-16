#![allow(unused)]

mod migrator;
use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::SchemaManager;

mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend() {
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

    let schema_manager = SchemaManager::new(db); // To investigate the schema

    // Migrator::refresh(db).await?;
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);

    // delete all
    let res: DeleteResult = chef::Entity::delete_many().exec(db).await?;
    let res: DeleteResult = bakery::Entity::delete_many().exec(db).await?;

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

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
