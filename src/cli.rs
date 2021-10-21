use clap::{App, Arg, SubCommand};

pub struct CLI {
    pub url: String,
    pub key: String,
    pub datasource_url: String,
    pub query: String,
    pub from: u64,
    pub to: u64
}

impl CLI {

    pub fn new() -> CLI {

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

        CLI{
            url: url.to_owned(),
            key: key.to_owned(),
            datasource_url: datasource_url.to_owned(),
            query: query.to_owned(),
            from,
            to
        }
    }
}