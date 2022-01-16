use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, insert_into, QueryDsl, QueryResult, RunQueryDsl, update};
use cicada_common::CicadaResult;
use crate::{Connection, ConnectionPool, DbResult, get_connection, result, User};
use crate::schema::user_notifications;

#[derive(Debug, Queryable, Serialize)]
pub struct UserNotifications {
    #[serde(skip)]
    id: i32,
    #[serde(skip)]
    user_id: i32,
    auth_login: bool,
    auth_password_change: bool,
    auth_attempt: bool,
    deploy_start: bool,
    deploy_finish: bool,
    deploy_fail: bool,
    #[serde(skip)]
    created_at: NaiveDateTime,
    #[serde(skip)]
    updated_at: NaiveDateTime
}

impl UserNotifications {

    fn from_user_raw(connection: &Connection, user: &User) -> QueryResult<Self> {
        user_notifications::dsl::user_notifications
            .filter(user_notifications::dsl::user_id.eq(user.id))
            .get_result(connection)
    }

    pub fn from_user(db: &ConnectionPool, user: &User) -> CicadaResult<Self> {
        result(
            match Self::from_user_raw(&get_connection(db)?, user) {
                Ok(notifications) => Ok(notifications),
                _ => {
                    NewUserNotifications::create(db, user.id)?;
                    Self::from_user_raw(&get_connection(db)?, user)
                }
            }
        )
    }

}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "user_notifications"]
pub struct NewUserNotifications {
    pub user_id: i32
}

impl NewUserNotifications {

    pub fn create(db: &ConnectionPool, user_id: i32) -> CicadaResult<usize> {
        result(
            insert_into(user_notifications::dsl::user_notifications)
                .values(&Self {
                    user_id
                })
                .execute(&get_connection(db)?)
        )
    }

}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "user_notifications"]
pub struct UpdateUserNotifications {
    auth_login: bool,
    auth_password_change: bool,
    auth_attempt: bool,
    deploy_start: bool,
    deploy_finish: bool,
    deploy_fail: bool
}

impl UpdateUserNotifications {

    pub fn update(&self, db: &ConnectionPool, user: &User) -> DbResult<usize> {
        UserNotifications::from_user(db, user)?;
        result(update(user_notifications::dsl::user_notifications.filter(user_notifications::dsl::user_id.eq(user.id))).set(self).execute(&get_connection(db)?))
    }

}
