pub mod clan;
pub mod event;
pub mod sample;

use crate::graphql::{clan::ClanQuery, event::EventQuery, sample::SampleQuery};

use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::{MergedObject, Schema};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(EventQuery, SampleQuery, ClanQuery);
