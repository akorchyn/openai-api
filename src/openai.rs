use ureq::{Agent, AgentBuilder};

pub struct Auth {
	pub api_key: String,
	pub organization: Option<String>,
}
#[allow(dead_code)]
impl Auth {
	pub fn new(api_key: &str) -> Auth {
		Auth { api_key: api_key.to_string(), organization: None }
	}

	pub fn from_env() -> Result<Self, String> {
		let api_key =
			std::env::var("OPENAI_API_KEY").map_err(|_| "Missing OPENAI_API_KEY".to_string())?;
		Ok(Self { api_key, organization: None })
	}
}

pub struct OpenAI {
	pub auth: Auth,
	pub api_url: String,
	pub(crate) agent: Agent,
}

#[allow(dead_code)]
impl OpenAI {
	pub fn new(auth: Auth, api_url: &str) -> OpenAI {
		OpenAI { auth, api_url: api_url.to_string(), agent: AgentBuilder::new().build() }
	}

	pub fn set_proxy(mut self, proxy: &str) -> OpenAI {
		let proxy = ureq::Proxy::new(proxy).unwrap();
		self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
		self
	}

	pub fn use_env_proxy(mut self) -> Result<OpenAI, String> {
		let mut proxy = std::env::var("http_proxy");
		if let Err(_) = proxy {
			proxy = std::env::var("https_proxy");
		}
		let proxy = proxy.map_err(|_| "Missing http_proxy or https_proxy".to_string())?;
		let proxy = ureq::Proxy::new(proxy).unwrap();
		self.agent = ureq::AgentBuilder::new().proxy(proxy).build();
		Ok(self)
	}
}

#[cfg(test)]
pub fn new_test_openai() -> OpenAI {
	let auth = Auth::from_env().unwrap();
	OpenAI::new(auth, "https://api.openai.com/v1/").use_env_proxy().unwrap()
}
