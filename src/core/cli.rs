use clap::{Arg, ArgAction, Command};

pub struct Cli;

impl Cli {
    pub fn parse() {
        let matches = Command::new("arb")
            .version("0.1.0")
            .author("Soham Zemse <???>")
            .about("Arbitrium CLI")
            .arg(
                Arg::new("address")
                    .help("Add a contract to the database")
                    .long("address")
                    .short('a')
                    .number_of_values(1)
                    .value_name("ADDRESS")
                    .required(false),
            )
            .arg(
                Arg::new("from")
                    .help("Start indexing from this block")
                    .long("from")
                    .short('f')
                    .number_of_values(1)
                    .value_name("FROM_BLOCK")
                    .required(false),
            )
            .arg(
                Arg::new("to")
                    .help("Stop indexing at this block")
                    .long("to")
                    .short('t')
                    .number_of_values(1)
                    .value_name("TO_BLOCK")
                    .required(false),
            )
            .arg(
                Arg::new("method")
                    .help("Method to run")
                    .long("method")
                    .short('m')
                    .number_of_values(1)
                    .value_name("METHOD")
                    .required(false),
            )
            .arg(
                Arg::new("run")
                    .help("Run the indexer")
                    .long("run")
                    .short('r')
                    .action(ArgAction::Set),
            )
            .get_matches();

        let binding = "0x0".to_string();
        let address = matches.get_one::<String>("address").unwrap_or(&binding);
        let from = matches.get_one::<u64>("from").unwrap_or(&0);
        let to = matches.get_one::<u64>("to").unwrap_or(&0);
        let binding = "arbtrace_block".to_string();
        let method = matches.get_one::<String>("method").unwrap_or(&binding);

        println!("Address: {}", address);
        println!("From: {}", from);
        println!("To: {}", to);
        println!("Method: {}", method);
    }
}
