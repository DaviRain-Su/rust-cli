// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{
    base64_opt, process_base64_decode, process_base64_encode, process_csv, process_genpass, text,
    Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
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
            process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.number,
                genpass_opts.symbol,
            )?;
        }
        SubCommand::Base64(base64_opt) => match base64_opt {
            base64_opt::Base64SubCommand::Base64Encode(base64_opts) => {
                process_base64_encode(&base64_opts.input, base64_opts.format)?;
            }
            base64_opt::Base64SubCommand::Base64Decode(base64_opts) => {
                process_base64_decode(&base64_opts.input, base64_opts.format)?;
            }
        },
        SubCommand::Text(text_opt) => match text_opt {
            text::TextSubCommand::Sign(text) => {
                println!("{:?}", text);
            }
            text::TextSubCommand::Verify(text) => {
                println!("{:?}", text);
            }
        },
    }

    Ok(())
}
