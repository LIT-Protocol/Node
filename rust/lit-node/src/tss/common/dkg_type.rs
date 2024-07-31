use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkgType {
    Standard = 1,
    RecoveryParty = 2,
}

impl DkgType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DkgType::Standard => "STANDARD",
            DkgType::RecoveryParty => "RECOVERY",
        }
    }

    pub fn is_to_be_backed_up(&self) -> bool {
        match self {
            DkgType::Standard => true,
            DkgType::RecoveryParty => false,
        }
    }
}

impl Display for DkgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
