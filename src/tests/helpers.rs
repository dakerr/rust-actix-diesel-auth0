#[cfg(test)]
pub mod tests {
    use crate::config::CONFIG;
    use crate::database::{init_pool, Pool};
    use diesel::pg::PgConnection;

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn get_pool() -> Pool<PgConnection> {
        init_pool::<PgConnection>(CONFIG.clone()).unwrap()
    }
}


