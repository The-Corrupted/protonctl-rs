pub mod github;
pub mod cmd;
pub mod list;

use cmd::{Actions, ProtonCtl};
use clap::Parser;

#[tokio::main]
async fn main() {
    let proton = ProtonCtl::parse();
    match &proton.actions {
        Actions::Install(install) => {
            println!("Install action specified: {:?}", install);
        }
        Actions::List(list) => {
            let releases = list::get_releases_paged(list.number, 1).await;
            match releases {
                Some(rel) => {
                    for r in rel.iter() {
                        println!("Release: {}", r.get_version());
                        println!("Download Url: {}", r.get_download_url());
                        println!("Body: {}", r.get_body());
                        println!("-----------------------");
                    }
                }
                None => {
                    println!("No releases found!");
                }
            }
        }
    }
}
