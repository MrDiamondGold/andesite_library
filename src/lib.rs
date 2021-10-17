use ureq::{Agent, AgentBuilder};
use serde::Deserialize;

pub struct UserKey {
    username: String,
    password: String,
}

impl UserKey {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Pages(Vec<String>);

pub struct Andesite {
    uri: String,
    user_key: UserKey,
    agent: Agent,
}

impl Andesite {
    pub fn new(uri: String, user_key: UserKey) -> Self {
        let agent = AgentBuilder::new()
            .timeout_read(std::time::Duration::from_secs(5))
            .timeout_write(std::time::Duration::from_secs(5))
            .build();

        Self {
            uri,
            user_key,
            agent,
        }
    }

    pub fn request_index(self) -> Pages {
        self.agent
            .get(&self.uri)
            .set("username", &self.user_key.username)
            .set("password", &self.user_key.password)
            .call().unwrap()
            .into_json().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Andesite, UserKey};

    fn get_uri() -> String {
        String::from("127.0.0.1:8000")
    }

    #[test]
    fn test_index() {
        let uri = get_uri();

        let andesite = Andesite::new(uri, UserKey::new("admin".into(), "password".into()));

        println!("{:?}", andesite.request_index());
    }
}
