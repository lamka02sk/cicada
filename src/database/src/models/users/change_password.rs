use cicada_common::CicadaResult;
use cicada_common::crypto::password::hash_password;
use crate::schema::users;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordForm {
    pub old_password: String,
    pub password: String
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct ChangePassword {
    pub password: String
}

impl From<&ChangePasswordForm> for CicadaResult<ChangePassword> {
    fn from(form: &ChangePasswordForm) -> Self {
        Ok(ChangePassword {
            password: hash_password(&form.password)?
        })
    }
}
