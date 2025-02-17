use constants::find_class::{FindClassRes, FIND_CLASS_URL};


pub async fn get_info() -> Result<FindClassRes, reqwest::Error> {
    let request = reqwest::get(FIND_CLASS_URL).await?;

    request.json::<FindClassRes>().await
}