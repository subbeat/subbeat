use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("My Super Program")
                          .version("0.0.2")
                          .about("Timeseries toolkit")
                          .arg(Arg::with_name("INPUT")
                          .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                               .get_matches();

    let input = matches.value_of("INPUT").unwrap();

    println!("input file for influxdb {}", input);
}
