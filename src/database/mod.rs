pub mod competition_events;
pub mod competitions;
pub mod events;
pub mod group_participation;
pub mod groups;
pub mod score_calculators;
pub mod scores;
pub mod season_competitions;
pub mod seasons;
pub mod teams;
pub mod users;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use serde_json::json;

use crate::{
    config::AppConfig,
    services::{
        audit::AuditService,
        users::{User, UsersService},
    },
};

use self::users::UserPermission;

const DATABASE_DEFINITION_SQL: &str = include_str!("../../database.sql");

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    /// Creates a new database connection pool
    pub fn new(config: &AppConfig) -> Self {
        let manager = SqliteConnectionManager::file(&config.database_file)
            .with_init(|c| c.execute_batch("PRAGMA busy_timeout = 60000; PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = 1; PRAGMA auto_vacuum = INCREMENTAL; PRAGMA recursive_triggers = 1;"));

        let pool = r2d2::Pool::new(manager).expect("Error occurred connecting to database");
        let conn = pool
            .get()
            .expect("Error occurred getting database connection for initialization");

        let db = Self { pool };

        let tables: &[&str] = &[
            "users",
            "sessions",
            "score_calculators",
            "seasons",
            "competitions",
            "season_competitions",
            "events",
            "competition_events",
            "groups",
            "group_participation",
            "teams",
            "group_scores",
            "team_scores",
            "log",
        ];
        let table_sql = format!("('{}')", tables.join("', '"));

        // check if tables exist
        let table_count: i64 = conn
            .prepare_cached(&format!("SELECT count() AS \"count\" FROM \"sqlite_master\" WHERE \"name\" IN {} AND \"type\" = 'table';", table_sql))
            .unwrap()
            .query_row([], |row| row.get("count"))
            .expect("Error occurred while checking for database initialization");
        let tables_exist = table_count == tables.len() as i64;

        if !tables_exist {
            // create tables
            conn.execute_batch(DATABASE_DEFINITION_SQL)
                .expect("Error occurred while running database initialization commands");

            let audit_service = AuditService::new(&db);

            audit_service.log(None, "init");

            let user_service = UsersService::new(&db, config);

            if user_service
                .get_user_by_name(&config.default_admin_username)
                .is_none()
            {
                let user_id = user_service
                    .create(&User {
                        id: None,
                        username: String::from(&config.default_admin_username),
                        new_password: Some(String::from(&config.default_admin_password)),
                        description: String::new(),
                        enabled: true,
                        permissions: UserPermission::MODIFY_SELF | UserPermission::USER_ADMIN,
                        permission_modify_self: true,
                        permission_user_admin: true,
                        permission_setup_admin: true,
                        permission_view_results: true,
                        permission_view_scores: true,
                        permission_enter_scores: true,
                        permission_view_registration: true,
                        permission_enter_registration: true,
                    })
                    .expect("Error occurred creating default admin user");

                audit_service.log_data(None, "default_user_created", json!({"user_id": user_id}));
            }
        }

        db
    }

    /// Gets an instance of the database connection pool
    pub fn get(&self) -> PooledConnection<SqliteConnectionManager> {
        self.pool
            .get()
            .expect("Error occurred getting database connection from connection pool")
    }
}
