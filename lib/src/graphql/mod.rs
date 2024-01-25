pub mod sample;
pub mod today;

use crate::graphql::{sample::SampleQuery, today::TodayQuery};

use async_graphql::{EmptySubscription};
use async_graphql::{MergedObject, Schema};

use self::today::TodayMutation;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SampleQuery, TodayQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(TodayMutation);
