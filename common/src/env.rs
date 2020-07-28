use crate::{Context, Result};
use dotenv::dotenv;
use std::env;

/// Mangles environment variables with a env dotfile
///
/// # Errors
/// Unable to handle io
#[inline]
pub fn setup_env() -> Result<()> {
    dotenv().context("Unable to setup environment").map(|_| ())
}

#[derive(Debug, Clone)]
/// Environment Variables required for the app
pub struct Env {
    /// Server to connect to with its port
    pub fqdn_with_port: String,
    /// A valid url which returns 200 when called
    pub url_returning_200: String,
    /// Whether the connection should be encrypted
    pub encrypted: bool,
}

impl Env {
    /// Creats the `Enc` structure using environment variables
    ///
    /// # Errors
    /// Fails if one or more env variables are missing
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self {
            fqdn_with_port: env::var("FQDN_WITH_PORT").context("FQDN_WITH_PORT must be set")?,
            url_returning_200: env::var("URL_RETURING_200")
                .context("URL_RETURING_200 must be set")?,
            encrypted: env::var("ENCRYPTED")
                .context("ENCRYPTED must be set")?
                .parse()
                .context("Unable to convert ENCRYPTED into bool")?,
        })
    }
}
