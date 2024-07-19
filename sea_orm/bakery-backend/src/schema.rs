use crate::entities::{prelude::*, *};
use async_graphql::{Context, Object};
use sea_orm::*;
pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    // For finding all bakeries
    async fn bakeries(&self, ctx: &Context<'_>) -> Result<Vec<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Bakery::find().all(db).await
    }

    // For finding one bakery by id
    async fn bakery(&self, ctx: &Context<'_>, id: i32) -> Result<Option<bakery::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Bakery::find_by_id(id).one(db).await
    }
}
