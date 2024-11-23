use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const DEFAULT_CACHE_FILE: &str = "~/jump.txt";
const MAX_MRU_ENTRIES: usize = 10;

pub struct MruCache {
    cache_file: String,
    paths: Vec<String>,
}

impl MruCache {
    pub fn load() -> MruCache {
        let cache_file = MruCache::expand_tilde(&MruCache::get_cache_file());
        let paths = MruCache::load_cache_file(&cache_file);

        MruCache { cache_file, paths }
    }

    pub fn get_all_paths(&self) -> Vec<String> {
        self.paths.clone()
    }

    pub fn get_matching_paths(&self, pattern: &str) -> Vec<String> {
        self.paths
            .iter()
            .filter(|p| p.contains(pattern))
            .cloned()
            .collect()
    }

    pub fn add_to_cache(&mut self, path: String) {
        self.paths.retain(|p| p != &path);
        self.paths.insert(0, path);
        if self.paths.len() > MAX_MRU_ENTRIES {
            self.paths.truncate(MAX_MRU_ENTRIES);
        }
    }

    pub fn write_mru_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.cache_file)
            .expect("Failed to open cache file for writing.");
        for line in &self.paths {
            writeln!(file, "{}", line).expect("Failed to write to cache file.");
        }
    }

    #[allow(clippy::lines_filter_map_ok)]
    fn load_cache_file(file: &String) -> Vec<String> {
        if !Path::new(file).exists() {
            return vec![];
        }

        let file = File::open(file).expect("Failed to open cache file.");
        BufReader::new(file)
            .lines()
            .filter_map(|line| line.ok())
            .collect()
    }

    fn get_cache_file() -> String {
        env::var("JUMP_CACHE").unwrap_or_else(|_| DEFAULT_CACHE_FILE.to_string())
    }

    fn expand_tilde(path: &str) -> String {
        match path.starts_with("~/") {
            false => path.to_string(),
            true => path.replacen("~", &MruCache::home_dir(), 1),
        }
    }

    fn home_dir() -> String {
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    }
}
