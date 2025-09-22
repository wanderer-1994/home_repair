use actor_auth::ActorAuth;
use argon2_hash::Argon2Hash;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use entity_type::HandymanId;
use error::{Error, Result};
use phonenumber::PhoneNumber;
use share_service_schema::handyman_account;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = handyman_account)]
pub struct HandymanAccount {
    pub id: HandymanId,
    pub phone_number: String,
    pub password_hash: String,
}

impl HandymanAccount {
    /// Verify if provided plain text password matches with hashed password
    pub fn verify_password(self, password: &str) -> Result<Self> {
        Argon2Hash::verify_password(password, &self.password_hash)
            .map_err(|_| Error::unauthenticated("Credentials not found"))?;
        Ok(self)
    }
}

impl HandymanAccount {
    pub async fn create(
        actor_auth: &ActorAuth,
        NewHandymanAccount {
            phone_number,
            password,
        }: NewHandymanAccount<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_god_or_admin()?;
        crate::check_password_safety(password)?;
        let password_hash = Argon2Hash::hash_password(password)?;
        let phone_number_str = crate::phone_number_to_e164_format(phone_number);

        if Self::phone_exist(&phone_number_str, conn).await? {
            return Err(Error::already_exists("Phone number already exist"));
        }

        diesel::insert_into(handyman_account::table)
            .values((
                handyman_account::phone_number.eq(&phone_number_str),
                handyman_account::password_hash.eq(password_hash),
            ))
            .returning(Self::as_select())
            .get_result::<Self>(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn phone_exist(
        e164_phone_number_str: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<bool> {
        diesel::select(diesel::dsl::exists(
            handyman_account::table
                .filter(handyman_account::phone_number.eq(e164_phone_number_str)),
        ))
        .get_result::<bool>(conn)
        .await
        .map_err(Error::from)
    }

    /// Find account by phone
    pub async fn find_by_phone_number(
        e164_phone_number_str: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>> {
        let account = handyman_account::table
            .filter(handyman_account::phone_number.eq(e164_phone_number_str))
            .select(Self::as_select())
            .first::<Self>(conn)
            .await
            .optional()?;
        Ok(account)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NewHandymanAccount<'a> {
    pub phone_number: &'a PhoneNumber,
    /// User input plain text password
    pub password: &'a str,
}
