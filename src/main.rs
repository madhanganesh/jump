use std::env::args;
use std::path::Path;

use jump::MruCache;

fn main() {
    let mut cache = MruCache::load();

    let args: Vec<String> = args().skip(1).collect();
    match args.len() {
        0 => print_paths(&cache.get_all_paths()),
        1 => handle_jump_arg(&args[0], &mut cache),
        _ => println!("Usage: jump <path | search term>"),
    }
}

fn handle_jump_arg(arg: &String, cache: &mut MruCache) {
    if Path::new(arg).exists() {
        handle_path(arg, cache);
        return;
    }

    let matchig_paths = cache.get_matching_paths(arg);
    if matchig_paths.is_empty() {
        eprintln!("no matchin paths");
        return;
    }

    if matchig_paths.len() == 1 {
        handle_path(&matchig_paths[0], cache);
        return;
    }

    print_paths(&cache.get_matching_paths(arg));
}

fn print_paths(paths: &[String]) {
    for path in paths {
        eprintln!("{}", path)
    }
}

fn handle_path(path: &String, cache: &mut MruCache) {
    cache.add_to_cache(path.clone());
    cache.write_mru_file();
    println!("{}", path);
}
