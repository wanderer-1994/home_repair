use crate::schema::customer_account;
use actor_auth::ActorAuth;
use argon2_hash::Argon2Hash;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use entity_type::CustomerId;
use error::{Error, Result};
use phonenumber::PhoneNumber;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = customer_account)]
pub struct CustomerAccount {
    pub id: CustomerId,
    pub phone_number: String,
    pub password_hash: String,
}

impl CustomerAccount {
    /// Verify if provided plain text password matches with hashed password
    pub fn verify_password(self, password: &str) -> Result<Self> {
        Argon2Hash::verify_password(password, &self.password_hash)
            .map_err(|_| Error::unauthenticated("Credentials not found"))?;
        Ok(self)
    }
}

impl CustomerAccount {
    pub async fn create(
        actor_auth: &ActorAuth,
        NewCustomerAccount {
            phone_number,
            password,
        }: NewCustomerAccount<'_>,
        conn: &mut AsyncPgConnection,
    ) -> Result<Self> {
        actor_auth.require_god_or_admin()?;
        typesafe::check_password_safety(password)?;
        let password_hash = Argon2Hash::hash_password(password)?;
        let phone_number_str = typesafe::phone_number_to_e164_format(phone_number);

        if Self::phone_exist(&phone_number_str, conn).await? {
            return Err(Error::already_exists("Phone number already exist"));
        }

        diesel::insert_into(customer_account::table)
            .values((
                customer_account::phone_number.eq(&phone_number_str),
                customer_account::password_hash.eq(password_hash),
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
            customer_account::table
                .filter(customer_account::phone_number.eq(e164_phone_number_str)),
        ))
        .get_result::<bool>(conn)
        .await
        .map_err(Error::from)
    }

    /// Get account by phone
    pub async fn find_by_phone_number(
        e164_phone_number_str: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>> {
        let account = customer_account::table
            .filter(customer_account::phone_number.eq(e164_phone_number_str))
            .select(Self::as_select())
            .first::<Self>(conn)
            .await
            .optional()?;
        Ok(account)
    }

    /// Load many accounts by ids
    pub async fn load_by_ids(
        // TODO: define read permission for customer account
        _actor_auth: &ActorAuth,
        ids: &[CustomerId],
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>> {
        let result = customer_account::table
            .filter(customer_account::id.eq_any(ids))
            .select(Self::as_select())
            .load::<Self>(conn)
            .await?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy)]
/// Raw new user account with raw password
pub struct NewCustomerAccount<'a> {
    pub phone_number: &'a PhoneNumber,
    /// User input plain text password
    pub password: &'a str,
}
