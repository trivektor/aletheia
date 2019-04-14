use super::RequestContext;
use crate::types::{Project, User};

graphql_object!(Project: RequestContext |&self| {
    description: "A hackathon project"

    field id(&executor) -> i32 {
        self.id
    }

    field name(&executor) -> &str {
        &self.name
    }

    field repository_url(&executor) -> &str {
        &self.repository_url
    }

    field slug(&executor) -> &str {
        &self.slug
    }

    field description(&executor) -> Option<&str> {
        match &self.description {
            Some(desc) => Some(desc.as_str()),
            None => None
        }
    }

    field contributors(&executor) -> Vec<User> {
        let database: &diesel::PgConnection = &executor.context().conn;
        self.contributors(database)
    }
});
