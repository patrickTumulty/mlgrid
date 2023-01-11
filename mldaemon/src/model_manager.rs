

const DEFAULT_MODELS_DIR: &str = "models/";

pub struct ModelManager {
    models_dir: String,
}

impl ModelManager {
    pub fn new() -> Self {
        return ModelManager {
            models_dir: String::from(DEFAULT_MODELS_DIR)
        };
    }
}
