mod tutor;

use async_trait::async_trait;
use serde::Serialize;
use tutor::Tutor;

#[async_trait]
pub trait Project {
    async fn authenticate(&self);
    async fn init(&self);
}

#[derive(Debug, Clone, Serialize)]
pub enum Lang {
    Python,
    Rust,
    JavaScript,
}

pub fn list() {
    println!("Available projects:");
    println!("{}", Tutor { lang: Lang::Python });
}

pub async fn init(project: &str) {
    let project: Box<dyn Project> = match project {
        "tutor" => Box::new(Tutor { lang: Lang::Python }),
        _ => {
            println!("Project not found");
            return;
        }
    };

    project.init().await;
}
