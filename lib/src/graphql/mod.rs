pub mod sample;
pub mod today;

use crate::graphql::{sample::SampleQuery, today::TodayQuery};

use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SampleQuery, TodayQuery);
