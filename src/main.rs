// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::cli::TextSubCommand;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_generator, process_genpass,
    process_http_serve, process_text_sign, process_text_verify, Base64SubCommand, HttpSubCommand,
    Opts, SubCommand, TextSignFormat,
};
use std::fs;
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::GenPass(genpass_opts) => {
            let password = process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.number,
                genpass_opts.symbol,
            )?;
            // Make sure the password have at least one of each type

            let passwords_string = String::from_utf8(password)?;
            println!("{}", passwords_string);

            let estimate = zxcvbn(&passwords_string, &[]).unwrap();
            eprintln!("password strength {}", estimate.score());
        }
        SubCommand::Base64(base64_opt) => match base64_opt {
            Base64SubCommand::Base64Encode(base64_opts) => {
                let encode = process_base64_encode(&base64_opts.input, base64_opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Base64Decode(base64_opts) => {
                let decode = process_base64_decode(&base64_opts.input, base64_opts.format)?;
                // TODO: decode data might not string,
                // so we need to handle this case(but for this example, we assume it is)
                println!("{}", String::from_utf8(decode)?);
            }
        },
        SubCommand::Text(text_opt) => match text_opt {
            TextSubCommand::Sign(text) => {
                let sig = process_text_sign(&text.input, &text.key, text.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(text) => {
                let verifyed = process_text_verify(&text.input, &text.key, text.format, &text.sig)?;
                println!("{}", verifyed);
            }
            TextSubCommand::GenKey(opts) => {
                let key = process_generator(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(opts) => match opts {
            HttpSubCommand::Serve(http_opts) => {
                println!("{:?}", http_opts);
                println!(
                    "http server is running on http://0.0.0.0:{}",
                    http_opts.port
                );
                process_http_serve(http_opts.path, http_opts.port).await?;
            }
        },
    }

    Ok(())
}
