use std::sync::Arc;

use postgres_es::default_postgress_pool;

use crate::config::Config;

use self::clan::ClanService;
use self::discord::DiscordService;
use self::event::EventService;
use self::jwt::JwtService;
use self::user::UserService;

pub mod clan;
pub mod discord;
pub mod event;
pub mod jwt;
pub mod user;

#[derive(Clone)]
pub struct AppServices {
    pub clan: Arc<ClanService>,
    pub event: Arc<EventService>,
    pub user: Arc<UserService>,
    pub discord: Arc<DiscordService>,
    pub jwt: Arc<JwtService>,
}

impl AppServices {
    pub async fn new(config: Config) -> Self {
        let db_pool = default_postgress_pool(config.db_url.as_str()).await;

        let clan_service = Arc::new(clan::ClanService {
            db_pool: db_pool.clone(),
        });

        let event_service = Arc::new(event::EventService {
            db_pool: db_pool.clone(),
        });

        let user_service = Arc::new(user::UserService {
            db_pool: db_pool.clone(),
        });

        let discord_service = Arc::new(discord::DiscordService::new(config.discord));

        let jwt_service = Arc::new(jwt::JwtService::new(config.jwt));

        Self {
            clan: clan_service,
            event: event_service,
            user: user_service,
            discord: discord_service,
            jwt: jwt_service,
        }
    }
}
