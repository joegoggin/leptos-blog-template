#[cfg(feature = "hydrate")]
use web_sys::Storage;

#[cfg(feature = "hydrate")]
pub struct LocalStorageUtil {
    local_storage: Option<Storage>,
}

#[cfg(feature = "hydrate")]
impl LocalStorageUtil {
    pub fn new() -> Self {
        use web_sys::window;

        let local_storage: Option<Storage> = match window() {
            Some(window) => match window.local_storage().ok().flatten() {
                Some(local_storage) => Some(local_storage),
                None => None,
            },
            None => None,
        };

        Self { local_storage }
    }

    pub fn get_item(&self, key: &str) -> Option<String> {
        match &self.local_storage {
            Some(local_storage) => local_storage.get_item(key).ok().flatten(),
            None => None,
        }
    }

    pub fn set_item(&self, key: &str, value: &str) {
        if let Some(local_storage) = &self.local_storage {
            local_storage.set_item(key, value);
        }
    }
}
