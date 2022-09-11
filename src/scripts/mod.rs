pub(crate) mod chars;
mod detect;
pub(crate) mod grouping;
mod joyo_kanji;
mod lang_mapping;
mod script;
mod simplified_chinese;
mod traditional_chinese;

pub use self::detect::detect_script;
pub use self::detect::{raw_detect_script, RawScriptInfo};
pub use self::script::Script;
