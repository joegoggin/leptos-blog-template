#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct DotEnv {
    pub database_url: String,
}

#[cfg(feature = "ssr")]
impl DotEnv {
    pub fn new() -> Self {
        use dotenv::dotenv;

        dotenv().ok();

        Self {
            database_url: Self::get_var_or_panic("DATABASE_URL"),
        }
    }

    fn get_var_or_panic(var_name: &str) -> String {
        use std::env;

        env::var(var_name).expect(&format!("{} environment variable is required.", var_name))
    }
}
