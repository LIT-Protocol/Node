use term_table::{Table, TableStyle};

use lit_core::config::{LitConfig, LitConfigBuilder};

use crate::error::Result;

pub trait LitCliConfig {
    fn default() -> Result<LitConfig>;
    fn must_default() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn default_ascii_table(&self) -> Table<'static>;
}

impl LitCliConfig for LitConfig {
    fn default() -> Result<Self> {
        <LitConfig as LitCliConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_default() -> LitConfig {
        <LitConfig as LitCliConfig>::default().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        if builder.key().is_none() {
            builder = builder.set_key(Some("cli".to_string()));
        }

        builder.build()
    }

    // Util

    fn default_ascii_table(&self) -> Table<'static> {
        let mut table = Table::new();
        table.style = TableStyle::simple();

        table
    }
}
