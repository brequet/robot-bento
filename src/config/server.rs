pub struct ServerConfig {
    pub port: u16,
}

pub fn load() -> ServerConfig {
    ServerConfig { port: 5325 }
}
