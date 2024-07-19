use async_graphql::Object;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }
}