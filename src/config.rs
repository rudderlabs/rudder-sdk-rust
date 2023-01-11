
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub max_flush_interval: usize,
    pub max_flush_size: usize,
    pub gzip: bool,
}
const DEFAULT_MAX_FLUSH_INTERVAL_IN_MILLIS: usize = 10000;
const DEFAULT_MAX_FLUSH_SIZE: usize = 10;
const DEFAULT_ALLOW_GZIP: bool = true;

impl Default for Config {
    fn default() -> Config {
        Config {
            max_flush_interval: DEFAULT_MAX_FLUSH_INTERVAL_IN_MILLIS,
            max_flush_size: DEFAULT_MAX_FLUSH_SIZE,
            gzip: DEFAULT_ALLOW_GZIP,
        }
    }
}