use crate::github;

const MAX_PER_PAGE: u8 = 50;

pub async fn get_releases_paged(mut number: u8, page: usize) -> Option<github::api::Releases> {
    if number > MAX_PER_PAGE {
        number = MAX_PER_PAGE
    }
    
    let releases_wrapped = github::api::releases(Some(number), Some(page)).await;
    let releases = match releases_wrapped {
        Ok(e) => e,
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(releases)
}
