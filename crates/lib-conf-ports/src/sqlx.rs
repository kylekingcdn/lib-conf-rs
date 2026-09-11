#[cfg(feature = "sqlx-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-postgres")))]
#[cfg_attr(feature = "sqlx-postgres", doc = "Contains a `lib-conf` port of [`sqlx::postgres::PgPoolOptions`].")]
pub mod postgres;
