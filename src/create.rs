use std::fs;
use std::io::Write;
use std::path::Path;

static SATYSFI_DOC_TYPES: [&str; 3] = ["stdja", "stdjabook", "stdjareport"];

static STDJA_CONTENT: &str = include_str!("../templates/main_stdja.saty");

static STDJABOOK_CONTENT: &str = include_str!("../templates/main_stdjabook.saty");

static STDJAREPORT_CONTENT: &str = include_str!("../templates/main_stdjareport.saty");

static LOCAL_CONTENT: &str = include_str!("../templates/local.satyh");

pub fn create(name: String, doc_type: String) {
    let path = Path::new(&name);
    if path.exists() {
        println!("{} already exists", name);
        std::process::exit(1);
    }
    match fs::create_dir(path) {
        Ok(_) => println!("{} created", name),
        Err(e) => println!("{} not created: {}", name, e),
    }
    std::env::set_current_dir(&name).unwrap();
    create_saty_file();
    write_saty_file(&doc_type);
}

fn create_saty_file() {
    let path = Path::new("main.saty");
    match fs::File::create(path) {
        Ok(_) => println!("main.saty created"),
        Err(e) => {
            println!("main.saty not created: {}", e);
            std::process::exit(1);
        }
    }
    let path = Path::new("local.satyh");
    match fs::File::create(path) {
        Ok(_) => println!("local.satyh created"),
        Err(e) => {
            println!("local.satyh not created: {}", e);
            std::process::exit(1);
        }
    }
}

fn write_saty_file(doc_type: &str) {
    let content = match doc_type {
        "stdja" => STDJA_CONTENT,
        "stdjabook" => STDJABOOK_CONTENT,
        "stdjareport" => STDJAREPORT_CONTENT,
        _ => {
            println!("{} is not supported", doc_type);
            std::process::exit(1);
        }
    };

    // fs::File::open は Read-Only なので、fs::File::create を使う
    // write main.saty
    let mut f = fs::File::create("main.saty").unwrap();
    f.write(content.as_bytes()).unwrap();

    // write local.satyh
    let mut f = fs::File::create("local.satyh").unwrap();
    f.write(LOCAL_CONTENT.as_bytes()).unwrap();
}
