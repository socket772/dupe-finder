use ahash::AHashMap;
use sha2::{Digest, Sha512};
use std::{
    borrow::Borrow, env::args, fs::File, io::Read, os::unix::fs::MetadataExt, time::Instant,
};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = args().collect();

    let mut hashmap = AHashMap::<String, String>::new();

    if args.len() != 3 {
        println!("Errore, passa 3 argomenti. cartella file_output");
        return;
    }

    println!("{}", args[1]);

    let walk_dir = WalkDir::new(args[1].clone()).follow_links(true);

    // Riempimento vettore di checksum/
    for file in walk_dir.into_iter().flatten() {
        // Controllo se posso accedere alla cartella
        if file.path().exists() && file.path().is_file() && file.metadata().unwrap().size() > 0 {
            let file_open = File::open(file.path());
            if file_open.is_ok() {
                let data = &mut Vec::<u8>::new();
                let read_result = file_open.unwrap().read_to_end(data);
                if read_result.is_ok() {
                    let mut hasher = Sha512::new();

                    hasher.update(data);
                    hashmap.insert(
                        file.path().to_str().unwrap().to_string(),
                        format!("{:X}", hasher.finalize()),
                    );
                }
            }
        }
    }

    // Aggiungere parte di controllo duplicati
}
