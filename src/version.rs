

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum BankFormatVersion {
    #[default]
    PresentDay,
    Resistance,
    ColdWarCrisis,
    XboxElite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BankReadOptions {
    pub format_version:            BankFormatVersion,
    pub require_checksum:          BankChecksumOption,
    pub allow_duplicates:          bool,
    pub allow_post_mature_version: bool,
    pub valid_entry_threshold:     i32, //yes. i32
    pub trim_unused_configs:       bool,
    pub respect_signedness_bug:    bool
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum  BankChecksumOption {
    Ignore,
    #[default]
    Read,
    Validate
}

impl BankReadOptions {
    fn normal() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: Default::default(),
            allow_duplicates: false,
            allow_post_mature_version: false,
            valid_entry_threshold: 0,
            trim_unused_configs: false,
            respect_signedness_bug: false,
        }
    }

    fn validated() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: BankChecksumOption::Validate,
            allow_duplicates: false,
            allow_post_mature_version: false,
            valid_entry_threshold: 0,
            trim_unused_configs: false,
            respect_signedness_bug: false,
        }
    }

    fn obfuscated() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: BankChecksumOption::Validate,
            allow_duplicates: true,
            allow_post_mature_version: true,
            valid_entry_threshold: 1,
            trim_unused_configs: true,
            respect_signedness_bug: true,
        }
    }
}

impl Default for BankReadOptions {
    fn default() -> Self { Self::normal() }
}

