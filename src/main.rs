use clap::{Parser, Subcommand};
mod create;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]

enum Commands {
    /// 新しく作成するためのコマンド
    New {
        name: String,
        #[clap(short, long, default_value = "stdja")]
        doc_type: String,
    },
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    match args.command {
        Commands::New { name, doc_type } => {
            println!("new {}", name);
            create::create(name, doc_type);
        }
        _ => {
            println!("unknown command");
        }
    }
}
