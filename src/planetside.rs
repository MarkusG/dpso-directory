#[derive(Debug)]
pub enum Faction {
    NS,
    NC,
    TR,
    VS
}

#[derive(Debug)]
pub enum Server {
    Apex,
    Cobalt,
    Connery,
    Emerald,
    Miller,
    Jaeger,
    SolTech
}

pub fn parse_faction(text: &str) -> Option<Faction> {
    match text.to_lowercase().as_str() {
        "ns" => Some(Faction::NS),
        "nc" => Some(Faction::NC),
        "tr" => Some(Faction::TR),
        "vs" => Some(Faction::VS),
        _ => None
    }
}

pub fn parse_server(text: &str) -> Option<Server> {
    match text.to_lowercase().as_str() {
        "apex" => Some(Server::Apex),
        "cobalt" => Some(Server::Cobalt),
        "connery" => Some(Server::Connery),
        "emerald" => Some(Server::Emerald),
        "miller" => Some(Server::Miller),
        "jaeger" => Some(Server::Jaeger),
        "soltech" => Some(Server::SolTech),
        _ => None
    }
}
