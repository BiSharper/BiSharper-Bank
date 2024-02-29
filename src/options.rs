

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
    pub valid_entry_threshold:     u32,
    pub decompress_lazily:         bool,
    pub trim_unused_configs:       bool,
    pub respect_signedness_bug:    bool,
    pub respect_long_name_bug:     bool
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum  BankChecksumOption {
    Ignore,
    #[default]
    Read,
    Validate
}

impl BankReadOptions {
    pub fn requires_version_header(&self) -> bool {
        match self.format_version {
            BankFormatVersion::PresentDay => true,
            BankFormatVersion::Resistance => true,
            BankFormatVersion::XboxElite => true,
            BankFormatVersion::ColdWarCrisis => false
        }
    }

    pub fn normal() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: Default::default(),
            allow_duplicates: false,
            allow_post_mature_version: false,
            valid_entry_threshold: 0,
            decompress_lazily: true,
            trim_unused_configs: false,
            respect_signedness_bug: false,
            respect_long_name_bug: false,
        }
    }

    pub fn validated() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: BankChecksumOption::Validate,
            allow_duplicates: false,
            allow_post_mature_version: false,
            valid_entry_threshold: 0,
            decompress_lazily: false,
            trim_unused_configs: false,
            respect_signedness_bug: false,
            respect_long_name_bug: false,
        }
    }

    pub fn obfuscated() -> Self {
        Self {
            format_version: Default::default(),
            require_checksum: BankChecksumOption::Validate,
            allow_duplicates: true,
            allow_post_mature_version: true,
            valid_entry_threshold: 1,
            decompress_lazily: true,
            trim_unused_configs: true,
            respect_signedness_bug: true,
            respect_long_name_bug: true,
        }
    }
}

impl Default for BankReadOptions {
    fn default() -> Self { Self::normal() }
}

