use std::path::PathBuf;

pub fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("Error getting cache dir")
        .join("aurcat/downloads")
}

pub fn get_package_cache_dir(package: &str) -> PathBuf {
    get_cache_dir().join(package)
}
