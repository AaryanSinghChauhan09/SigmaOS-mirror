#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorEntry {
    pub url: String,
    pub country_code: String,
    pub ping_ms: u32,
}

#[derive(Debug, Default)]
pub struct MirrorManager {
    mirrors: Vec<MirrorEntry>,
}

impl MirrorManager {
    pub fn new() -> Self { Self { mirrors: Vec::new() } }
    pub fn add_mirror(&mut self, url: &str, country: &str, ping: u32) {
        self.mirrors.push(MirrorEntry {
            url: url.into(),
            country_code: country.into(),
            ping_ms: ping,
        });
    }
    pub fn fastest_mirror(&self) -> Option<&MirrorEntry> {
        self.mirrors.iter().min_by_key(|m| m.ping_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mirror_manager_fastest_aaa() {
        // Arrange
        let mut mgr = MirrorManager::new();
        mgr.add_mirror("https://us.repo.sigmaos.org", "US", 110);
        mgr.add_mirror("https://in.repo.sigmaos.org", "IN", 15);
        mgr.add_mirror("https://eu.repo.sigmaos.org", "DE", 85);
        // Act
        let fastest = mgr.fastest_mirror().expect("Fastest mirror should exist");
        // Assert
        assert_eq!(fastest.country_code, "IN");
        assert_eq!(fastest.ping_ms, 15);
    }
}
