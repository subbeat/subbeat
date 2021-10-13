use clap::{App, Arg, SubCommand};

mod types;

#[tokio::main]
async fn main() -> types::Result<()> {
    let matches = App::new("subbeat")
        .version("0.0.2")
        .about("Timeseries toolkit")
        .arg(
            Arg::with_name("GRAFANA_API_KEY")
                .help("Grafna API Key. Go to http://<grafana-url>/org/apikeys to get one")
                .required(true)
                .index(1),
        )
        .get_matches();

    // eyJrIjoiWnRRMTNmcGpvTHNPb3UzNzdUNUphRm53Rk9tMTNzOTQiLCJuIjoic3ViYmVhdC10ZXN0IiwiaWQiOjF9
    let input = matches.value_of("GRAFANA_API_KEY").unwrap();

    println!("input file for influxdb {}", input);
}
