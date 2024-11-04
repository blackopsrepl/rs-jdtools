use clap::Parser;
use std::path::PathBuf;

use jdtools::extract::extract_markdown_files_recursive;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Vittorio Distefano",
    about = "validates your resume against a jd"
)]

struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(name = "validate", about = "validates your resume against a jd")]
    Validate(ValidateOpts),

    #[clap(
        name = "letter",
        about = "writes cover letter for a jd, from your resume"
    )]
    Letter(LetterOpts),
}

#[derive(Parser)]
struct LetterOpts {
    #[clap(short, long, help = "path to your resume")]
    resume: String,
    #[clap(short, long, help = "path to your jd")]
    jd: String,
}

#[derive(Parser)]
struct ValidateOpts {
    #[clap(short, long, help = "path to your resume")]
    resume: String,
    #[clap(short, long, help = "path to your jd")]
    jd: String,
}

fn main() {
    let args: Opts = Opts::parse();
    match args.subcmd {
        SubCommand::Validate(_validate_opts) => {
            println!("validation not implemented yet");
        }

        SubCommand::Letter(letter_opts) => {
            let _resume = PathBuf::from(letter_opts.resume);
            let jd = PathBuf::from(letter_opts.jd);
            let output = extract_markdown_files_recursive(&jd);
            println!("{:#?}", output);
            //TODO: implement letter generation
            println!("letter generation not implemented yet");
        }
    }
}
