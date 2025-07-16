use std::fs;

use urldecode::url_decode;
use owo_colors::OwoColorize;
use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(version, about, long_about="It takes strings like `https%3A%2F%2Fwww.google.com` and turns it into strings like `https://www.google.com`")]
struct Cli{
    #[arg(help = "URL encoded string")]
    content: Option<String>,

    #[arg(short, long, help="decode file name", value_hint = ValueHint::FilePath)]
    name: Option<std::path::PathBuf>,

    #[arg(short, long, help="decode file content", value_hint = ValueHint::FilePath)]
    file: Option<std::path::PathBuf>,

    #[arg(short, long, help="output file", value_hint = ValueHint::FilePath)]
    output: Option<std::path::PathBuf>,
}

fn main(){
    let cli = Cli::parse();

    let provided_args = [
        !cli.content.is_none(),
        cli.name.is_some(),
        cli.file.is_some()
    ].iter().filter(|&&x| x).count();

    if provided_args > 1 {
        eprintln!("{}","only one parameter from `content`, `name`, and `file` can be used simultaneously.".red());
        return;
    }

    if !cli.content.is_none(){
        let res = url_decode(&cli.content.unwrap());
        if res.is_err(){
            eprintln!("{}", format!("error: {}", res.err().unwrap()).red());
            return;
        }

        if let Some(output_path) = cli.output{
            let temp = std::fs::write(&output_path, res.unwrap().as_bytes());
            if temp.is_err(){
                eprintln!("{}", format!("error: {}", temp.err().unwrap()).red());
            }else{
                println!("{}", format!("output file successfully! path = `{}`", output_path.display()).green());
            }
            return;
        }

        println!("{}", res.unwrap().green());
    }

    if let Some(input_path) = cli.name{
        if cli.output.is_some(){
            eprintln!("{}", "`--output` parameter will not work, because of `--name` you have added".yellow());
        }

        let dec = url_decode(input_path.file_name().unwrap().to_str().unwrap());
        if dec.is_err(){
            eprintln!("{}", format!("decoding file name error: {}", dec.unwrap()).red());
            return;
        }
        let mut output_path = input_path.clone();
        output_path.set_file_name(dec.unwrap());
        let temp = fs::rename(&input_path, &output_path);
        if temp.is_err(){
            eprintln!("{}", format!("failed to rename the file `{}`", input_path.display()).red());
            return;
        }
        println!("{}", "successfully renamed the file".green());
        return;
    }

    if let Some(input_path) = cli.file{
        let content = fs::read_to_string(&input_path);
        if content.is_err(){
            eprintln!("{}", format!("failed to read the file `{}`", input_path.display()).red());
            return;
        }

        let decoded = url_decode(&content.unwrap());
        if decoded.is_err(){
            eprintln!("{}", format!("decoding file content error: {}", decoded.unwrap()).red());
            return;
        }

        if let Some(output_path) = cli.output{
            let temp = fs::write(&output_path, decoded.unwrap().as_bytes());
            if temp.is_err() {
                eprintln!("{}", format!("failed to write the file `{}`", output_path.display()).red());
                return;
            }

            println!("{}", "output file successfully".green());
        }else{
            let temp = fs::write("a_decoded.dec", decoded.unwrap().as_bytes());
            if temp.is_err() {
                eprintln!("{}", format!("failed to write the file `{}`", "a_decoded.dec").red());
                return;
            }

            println!("{}", "output file `a_decoded.dec`".green());
        }

        return;
    }

    
}