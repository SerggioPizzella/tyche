use clap::Parser;

#[derive(Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(env)]
    pub database_url: String,

    /// The port to listen on.
    /// Defaults to 8080.
    #[clap(env, default_value = "8080")]
    pub port: u16,
}
