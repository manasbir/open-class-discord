use anyhow::{Context, Result};
use types::{FindClassRes, Property, FIND_CLASS_URL};

pub mod types;

pub async fn get_classes() -> Result<Vec<Property>> {
    Ok(reqwest::get(FIND_CLASS_URL)
        .await
        .context("Failed to fetch data")?
        .json::<FindClassRes>()
        .await
        .context("Failed to parse JSON")?
        .data
        .properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
