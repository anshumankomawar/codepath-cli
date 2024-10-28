use core::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use keyring::Entry;
use serde::Serialize;

use crate::git::create_repository_from_template;

use super::{Lang, Project};

#[derive(Debug, Clone, Serialize)]
pub struct Tutor {
    pub lang: Lang,
}

impl Display for Tutor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "- Tutor")
    }
}

#[async_trait]
impl Project for Tutor {
    async fn authenticate(&self) {
        todo!()
    }

    async fn init(&self) {
        let org = "anshumankomawar";
        let template_repo = "template";
        let new_repo_name = "usertemplate";
        let entry = Entry::new("codepath", "auth").unwrap();
        let user_token = entry.get_password().unwrap();

        create_repository_from_template(org, template_repo, new_repo_name, user_token.as_str())
            .await
            .expect("Failed to create repository from template");
    }
}
