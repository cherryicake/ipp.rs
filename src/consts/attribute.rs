//!
//! IPP attribute names
//!

// enum_primitive only works with integer values, so these will have to remain as constants

pub const ATTRIBUTES_CHARSET: &'static str = "attributes-charset";
pub const ATTRIBUTES_NATURAL_LANGUAGE: &'static str = "attributes-natural-language";
pub const CHARSET_CONFIGURED: &'static str = "charset-configured";
pub const CHARSET_SUPPORTED: &'static str = "charset-supported";
pub const COMPRESSION_SUPPORTED: &'static str = "compression-supported";
pub const DOCUMENT_FORMAT_DEFAULT: &'static str = "document-format-default";
pub const DOCUMENT_FORMAT_SUPPORTED: &'static str = "document-format-supported";
pub const GENERATED_NATURAL_LANGUAGE_SUPPORTED: &'static str = "generated-natural-language-supported";
pub const IPP_VERSIONS_SUPPORTED: &'static str = "ipp-versions-supported";
pub const NATURAL_LANGUAGE_CONFIGURED: &'static str = "natural-language-configured";
pub const OPERATIONS_SUPPORTED: &'static str = "operations-supported";
pub const PDL_OVERRIDE_SUPPORTED: &'static str = "pdl-override-supported";
pub const PRINTER_IS_ACCEPTING_JOBS: &'static str = "printer-is-accepting-jobs";
pub const PRINTER_MAKE_AND_MODEL: &'static str = "printer-make-and-model";
pub const PRINTER_NAME: &'static str = "printer-name";
pub const PRINTER_STATE: &'static str = "printer-state";
pub const PRINTER_STATE_REASONS: &'static str = "printer-state-reasons";
pub const PRINTER_UP_TIME: &'static str = "printer-up-time";
pub const PRINTER_URI: &'static str = "printer-uri";
pub const QUEUED_JOB_COUNT: &'static str = "queued-job-count";
pub const URI_AUTHENTICATION_SUPPORTED: &'static str = "uri-authentication-supported";
pub const URI_SECURITY_SUPPORTED: &'static str = "uri-security-supported";
pub const JOB_ID: &'static str = "job-id";
pub const JOB_NAME: &'static str = "job-name";
pub const JOB_STATE: &'static str = "job-state";
pub const JOB_STATE_REASONS: &'static str = "job-state-reasons";
pub const JOB_URI: &'static str = "job-uri";
pub const LAST_DOCUMENT: &'static str = "last-document";
pub const REQUESTING_USER_NAME: &'static str = "requesting-user-name";
pub const STATUS_MESSAGE: &'static str = "status-message";
pub const REQUESTED_ATTRIBUTES: &'static str = "requested-attributes";
pub const SIDES_SUPPORTED: &'static str = "sides-supported";
pub const OUTPUT_MODE_SUPPORTED: &'static str = "output-mode-supported";
pub const COLOR_SUPPORTED: &'static str = "color-supported";
pub const COPIES_SUPPORTED: &'static str = "copies-supported";
pub const COLOR_MODE_SUPPORTED: &'static str = "color-mode-supported";
pub const PRINT_COLOR_MODE_SUPPORTED: &'static str = "print-color-mode-supported";
