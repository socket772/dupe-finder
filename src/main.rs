use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::{
    borrow::{Borrow, BorrowMut},
    env::args,
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    time::Instant,
};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct Elemento {
    checksum: String,
    // lista_percorsi: Vec<String>,
}

fn main() {
    // argomento
    let args: Vec<String> = args().collect();

    // hashmap con la lista di tutti i file e i loro checksum
    // 0 = percorso file
    // 1 = checksum
    let mut hashmap = AHashMap::<String, String>::new();

    // hashmap con i checksum e file duplicati
    // 0 = checksum
    // 1 = array percorsi file
    let mut hashmap_final = AHashMap::<String, Vec<String>>::new();

    // controllo numero di argomenti
    if args.len() != 3 {
        println!("Errore, passa 3 argomenti. cartella file_output");
        return;
    }

    // lista di tutti i file in una cartella ()
    let walk_dir = WalkDir::new(args[1].clone()).follow_links(true);

    // crea file di log
    let mut file_writer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args[2].clone())
        .unwrap();

    let mut contatore: u32 = 0;

    // Riempimento vettore di checksum/
    for file in walk_dir.into_iter().flatten() {
        // Controllo se posso accedere alla cartella
        if file.path().exists() && file.path().is_file() && file.metadata().unwrap().size() > 0 {
            contatore += 1;
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

    println!("Numero file validi trovati{}", contatore);

    // Controllo dei duplicati
    let mut benchmark = Instant::now();
    for elemento_1 in hashmap.borrow() {
        for elemento_2 in hashmap.borrow() {
            if (elemento_1.0 != elemento_2.0) && (elemento_1.1 == elemento_2.1) {
                if hashmap_final.contains_key(elemento_2.1) {
                    hashmap_final
                        .get_mut(elemento_2.1)
                        .unwrap()
                        .push(elemento_2.0.to_string());
                } else {
                    hashmap_final.insert(elemento_2.1.to_string(), vec![elemento_2.0.to_string()]);
                }
                // let _ = writeln!(file_writer, "{} == {}", elemento_1.0, elemento_2.0);
            }
        }
    }
    println!("Cerca duplicati: {}ns", benchmark.elapsed().as_nanos());

    benchmark = Instant::now();
    // Deduplicazione elementi nell'array dei file
    for elemento in hashmap_final.borrow_mut() {
        elemento.1.sort();
        elemento.1.dedup();
    }
    println!("Deduplica entrate: {}ns", benchmark.elapsed().as_nanos());

    benchmark = Instant::now();
    // Salva output
    for elemento in hashmap_final {
        let _ = writeln!(file_writer, "{} => {:?}", elemento.0, elemento.1);
    }
    println!("Salva dati: {}ns", benchmark.elapsed().as_nanos());
}
