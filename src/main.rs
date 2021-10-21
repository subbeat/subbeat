use clap::{App, Arg, SubCommand};
use subbeat::grafana_service;

mod types;

#[tokio::main]
async fn main() -> types::Result<()> {
    let matches = App::new("subbeat")
        .version("0.0.2")
        .about("Timeseries toolkit")
        .arg(
            Arg::with_name("GRAFANA_URL")
                .help("URL to your Grafana instance")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("GRAFANA_API_KEY")
                .help("Grafna API Key. Go to http://<grafana-url>/org/apikeys to get one")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("datasource_url")
                .help("relative path to datasource")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("query")
                .help("your query to datasource")
                .required(true)
                .index(4),
        )
        .arg(
            Arg::with_name("from")
                .help("timestamp")
                .required(true)
                .index(5),
        )
        .arg(
            Arg::with_name("to")
                .help("timestampt")
                .required(true)
                .index(6),
        )
        .get_matches();

    let url = matches.value_of("GRAFANA_URL").unwrap();
    let key = matches.value_of("GRAFANA_API_KEY").unwrap();
    let datasource_url = matches.value_of("datasource_url").unwrap();
    let query = matches.value_of("query").unwrap();
    let from = matches.value_of("from").unwrap().parse().unwrap();
    let to = matches.value_of("to").unwrap().parse().unwrap();
    

    let gs = grafana_service::GrafanaService::new(url.to_string(), key.to_string());

    // gs.test_connection().await?;
    // gs.get_datasources().await?;
    // "http://localhost:3000/d/YeBxHjzWz/starter-app-stats?editPanel=2&orgId=1"
    let r = gs
        .extract_metrics(
            datasource_url,
            query,
            from,
            to,
            15,
        )
        .await?;

    let key = r.keys().nth(0).unwrap();
    println!("{}", key);

    let vs = &r[key];
    for (t, v) in vs.iter() {
        println!("{}\t{}", t, v);
    }

    Ok(())
}
