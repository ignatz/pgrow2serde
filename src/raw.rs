use std::error::Error;
use std::ops::Deref;
use tokio_postgres::types::{FromSql, Type};

/// The raw bytes of a value, allowing "conversion" from any postgres type.
///
/// This type intentionally cannot be converted from `NULL`, and attempting to
/// do so will result in an error. Instead, use `Option<Raw>`.
pub struct Raw<'a> {
    pub ty: Type,
    pub bytes: &'a [u8],
}

impl<'a> FromSql<'a> for Raw<'a> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            ty: ty.clone(),
            bytes: raw,
        })
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }
}

impl<'a> Deref for Raw<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

pub struct Boolean(bool);

impl<'a> FromSql<'a> for Boolean {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        match ty.name() {
            "int4" => Ok(Self(i32::from_sql(ty, raw)? > 0)),
            "int8" => Ok(Self(i64::from_sql(ty, raw)? > 0)),
            _ => Ok(Self(bool::from_sql(ty, raw)?)),
        }
    }

    fn accepts(ty: &Type) -> bool {
        match ty.name() {
            "bool" | "int4" | "int8" => true,
            _ => false,
        }
    }
}

impl Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
