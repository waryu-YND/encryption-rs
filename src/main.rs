mod lib;

use clap::Clap;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process;

#[derive(Clap)]
#[clap(
    name = "encryption",
    version = "0.1.0",
    author = "Waryu-YND",
    about = "Encrypt and decrypt your file."
)]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(
        version = "0.1.0",
        author = "Waryu-YND",
        about = "Encrypt your file.",
    )]
    Encrypt(Encrypt),

    #[clap(
        version = "0.1.0",
        author = "Waryu-YND",
        about = "Decrypt your file.",
    )]
    Decrypt(Decrypt),
}

#[derive(Clap)]
struct Encrypt {
    #[clap(
        about = "File to be encrypted."
    )]
    file_path: PathBuf,

    #[clap(
        short,long,
        about = "Output destination of encrypting file",
    )]
    output: Option<PathBuf>,

    #[clap(
        short,long,
        about = "ID to be used for encryption",
    )]
    id: Option<String>,

    #[clap(
        short,long,
        about = "length of ID",
    )]
    length: Option<usize>,
}

#[derive(Clap)]
struct Decrypt {
    #[clap(
        about = "The ID generated during encryption that is required for decryption."
    )]
    id: String,

    #[clap(
        about = "File to be decrypted."
    )]
    file_path: PathBuf,

    #[clap(
        short,long,
        about = "Output destination of decrypting file.",
    )]
    output: Option<PathBuf>
}

fn error_println<T,E: std::fmt::Display>(e: E) -> T {
    eprintln!("{}",e);
    process::exit(1);
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.sub_cmd {

        SubCommand::Encrypt(args) => {

            // Read normal file.
            let mut file = File::open(&args.file_path).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to open {}",args.file_path.to_string_lossy()))
            });
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap_or_else(|_e| {
                error_println(format!("Error: Failed to read {}",args.file_path.to_string_lossy()))
            });

            // Run to encrypt.
            let id_length = args.length.unwrap_or(20);
            let id = lib::EncryptionId::new(args.id,id_length).unwrap_or_else(error_println);
            let mut encryption = lib::Encryption::new(contents,id).unwrap_or_else(error_println);
            encryption.encrypt();
            println!("encryption ID: {}",encryption.id.string);
            println!("The ID is required for decryption.");
            println!("If you forgot it, Can't decrypt. Be careful.");

            // Write encryption file.
            let output_path = args.output.unwrap_or(args.file_path);
            let mut file = File::create(&output_path).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to open {}",output_path.to_string_lossy()))
            });
            file.write_all(&encryption.contents).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to write {}",output_path.to_string_lossy()))
            });

        }

        SubCommand::Decrypt(args) => {

            // Read normal file.
            let mut file = File::open(&args.file_path).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to open {}",args.file_path.to_string_lossy()))
            });
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap_or_else(|_e| {
                error_println(format!("Error: Failed to read {}",args.file_path.to_string_lossy()))
            });

            // Run to encrypt.
            let id = lib::EncryptionId::new(Some(args.id),20).unwrap_or_else(error_println);
            let mut encryption = lib::Encryption::new(contents,id).unwrap_or_else(error_println);
            encryption.decrypt();

            // Write encryption file.
            let output_path = args.output.unwrap_or(args.file_path);
            let mut file = File::create(&output_path).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to open {}",output_path.to_string_lossy()))
            });
            file.write_all(&encryption.contents).unwrap_or_else(|_e|{
                error_println(format!("Error: Failed to write {}",output_path.to_string_lossy()))
            });

        }
    }
}

