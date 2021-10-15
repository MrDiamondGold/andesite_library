use ureq::{Agent, AgentBuilder};

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

    pub fn request_index(self) -> String {
        self.agent
            .get(&self.uri)
            .set("username", &self.user_key.username)
            .set("password", &self.user_key.password)
            .call().unwrap()
            .into_string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use crate::{Andesite, UserKey};

    fn get_uri() -> String {
        dotenv().ok();

        env::var("andesite_uri").expect("Must specifiy an andesite_env in env").into()
    }

    #[test]
    fn test_index() {
        let uri = get_uri();

        let andesite = Andesite::new(uri, UserKey::new("admin".into(), "password".into()));

        assert_eq!("{\"hello\": \"world\"}", andesite.request_index());
    }
}
