extern crate structopt;

use std::io::{self, Write};
use ulid::Ulid;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of ULIDs to generate
    #[structopt(short = "n", long = "count", default_value = "1")]
    count: u32,
    /// ULIDs for inspection
    #[structopt(conflicts_with = "count")]
    ulids: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();

    if !opt.ulids.is_empty() {
        inspect(&opt.ulids);
    } else {
        generate(opt.count);
    }
}

fn generate(count: u32) {
    let stdout = io::stdout();
    let mut locked = stdout.lock();
    for _ in 0..count {
        writeln!(&mut locked, "{}", Ulid::new()).unwrap();
    }
}

fn inspect(values: &[String]) {
    for val in values {
        let ulid = Ulid::from_string(&val);
        match ulid {
            Ok(ulid) => {
                let upper_hex = format!("{:X}", ulid.0);
                println!(
                    "
REPRESENTATION:

  String: {}
     Raw: {}

COMPONENTS:

       Time: {}
  Timestamp: {}
    Payload: {}
",
                    ulid.to_string(),
                    upper_hex,
                    ulid.datetime().to_rfc2822(),
                    ulid.timestamp_ms(),
                    upper_hex.chars().skip(6).collect::<String>()
                );
            }
            Err(e) => {
                println!("{} is not a valid ULID: {}", val, e);
            }
        }
    }
}
