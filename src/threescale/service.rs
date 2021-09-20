use serde::{Deserialize, Serialize};

use super::{Credentials, MappingRule};
use crate::util::glob::GlobPatternSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Production,
    Staging,
    Sandbox,
    #[serde(other)]
    Unknown,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Production => "production",
            Self::Staging => "staging",
            Self::Sandbox => "sandbox",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(default)]
    pub environment: Environment,
    pub token: String,
    #[serde(default)]
    pub authorities: GlobPatternSet,
    pub credentials: Credentials,
    pub mapping_rules: Vec<MappingRule>,
}

impl Service {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    // Allow dead code until we have the logic asking for the environment
    // in the configuration retrieval subsystem.
    #[allow(dead_code)]
    pub fn environment(&self) -> &str {
        self.environment.as_str()
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    pub fn mapping_rules(&self) -> &Vec<MappingRule> {
        self.mapping_rules.as_ref()
    }

    pub fn match_authority(&self, authority: &str) -> bool {
        self.authorities.is_match(authority)
    }
}
