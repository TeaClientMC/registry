use super::structs::GitHosts;
use std::fmt::{Display, Formatter};

impl GitHosts {
    pub const VARIANTS: &'static [GitHosts] = &[Self::Github];
}

impl Display for GitHosts {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}
