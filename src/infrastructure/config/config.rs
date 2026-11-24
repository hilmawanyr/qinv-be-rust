pub struct Config {
    pub internal_enc_duration: u64,
    pub internal_enc_id: String,
    pub internal_enc_key: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            internal_enc_duration: std::env::var("INTERNAL_ENC_DURATION")?.parse()?,
            internal_enc_id: std::env::var("INTERNAL_ENC_ID")?,
            internal_enc_key: std::env::var("INTERNAL_ENC_KEY")?,
        })
    }
}
