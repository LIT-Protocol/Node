use async_std::fs;
use async_std::path::Path;

use crate::env::ENV_CACHE_PATH_KEY;
use crate::error::unexpected::Unexpected;
use crate::error::Result;

#[allow(dead_code)]
pub(crate) async fn cache_create_path(cache_path: &Path) -> Result<()> {
    if !cache_path.exists().await {
        fs::create_dir_all(&cache_path).await.expect_or_err(
            format!(
                "failed to create cache dir: {} \
                (hint: you can change the path by setting {})",
                cache_path.to_str().unwrap(),
                ENV_CACHE_PATH_KEY
            )
            .as_str(),
        )?;
    }

    Ok(())
}
