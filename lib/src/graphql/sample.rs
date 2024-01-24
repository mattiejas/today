use async_graphql::{Context, Object};

use crate::error::AppResult;

#[derive(Default)]
pub struct SampleQuery;

#[Object]
impl SampleQuery {
    async fn sample(&self, _ctx: &Context<'_>) -> AppResult<String> {
        Ok("sample".to_string())
    }
}
