use std::fmt::{Display, Formatter};

use async_graphql::{InputValueError, Scalar, ScalarType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Uuid(uuid::Uuid);

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[Scalar]
impl ScalarType for Uuid {
    fn parse(value: async_graphql::Value) -> Result<Uuid, InputValueError<Uuid>> {
        let uuid = match value {
            async_graphql::Value::String(s) => uuid::Uuid::parse_str(&s)?,
            _ => return Err("Invalid UUID".into()),
        };

        Ok(Uuid(uuid))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.to_string())
    }
}

impl Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}
