use std::sync::Arc;

use postgres_es::default_postgress_pool;

use crate::config::Config;

use self::jwt::JwtService;
use self::user::UserService;

pub mod jwt;
pub mod user;

#[derive(Clone)]
pub struct AppServices {
    pub user: Arc<UserService>,
    pub jwt: Arc<JwtService>,
}

impl AppServices {
    pub async fn new(config: Config) -> Self {
        let db_pool = default_postgress_pool(config.db_url.as_str()).await;

        let user_service = Arc::new(user::UserService {
            db_pool: db_pool.clone(),
        });

        let jwt_service = Arc::new(jwt::JwtService::new(config.jwt));

        Self {
            user: user_service,
            jwt: jwt_service,
        }
    }
}
