use clap::{App, Arg, SubCommand};
use subbeat::{datasources::grafana::Grafana, types::{DatasourceConfig, GrafanaConfig, InfluxConfig, PrometheusConfig, QueryConfig}};

pub struct CLI {
    pub query_config: QueryConfig,
}

impl CLI {
    // TODO: convert to result
    pub fn new() -> CLI {
        let matches = App::new("subbeat")
            .version("0.0.5")
            .about("Timeseries toolkit")
            .subcommand(
                SubCommand::with_name("grafana")
                    .about("Use Grafana as datasource")
                    .arg(
                        Arg::with_name("GRAFANA_URL")
                            .help("URL to your Grafana instance")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("GRAFANA_API_KEY")
                            .help(
                                "Grafna API Key. Go to http://<grafana-url>/org/apikeys to get one",
                            )
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
                    .arg(
                        Arg::with_name("step")
                            .help("aggregation step")
                            .required(true)
                            .index(7),
                    ),
            )
            .subcommand(
                SubCommand::with_name("prometheus")
                    .about("Use prometheus API as datasource")
                    .arg(
                        Arg::with_name("PROM_URL")
                            .help("URL to your Prometheus instance")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("query")
                            .help("your query to datasource")
                            .required(true)
                            .index(2),
                    )
                    .arg(
                        Arg::with_name("from")
                            .help("timestamp")
                            .required(true)
                            .index(3),
                    )
                    .arg(
                        Arg::with_name("to")
                            .help("timestampt")
                            .required(true)
                            .index(4),
                    )
                    .arg(
                        Arg::with_name("step")
                            .help("aggregation step")
                            .required(true)
                            .index(5),
                    ),
            )
            .subcommand(
                SubCommand::with_name("influx")
                    .about("Use influxdb API as datasource")
                    .arg(
                        Arg::with_name("INFLUX_URL")
                            .help("URL to your influxdb instance")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("ORG_ID")
                            .help("URL to your influxdb instance")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("TOKEN")
                            .help("URL to your influxdb instance")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("query")
                            .help("your flux query to datasource")
                            .required(true)
                            .index(2),
                    )
                    .arg(
                        Arg::with_name("from")
                            .help("timestamp")
                            .required(true)
                            .index(3),
                    )
                    .arg(
                        Arg::with_name("to")
                            .help("timestampt")
                            .required(true)
                            .index(4),
                    )
                    .arg(
                        Arg::with_name("step")
                            .help("aggregation step")
                            .required(true)
                            .index(5),
                    ),
            )
            .get_matches();

        if let Some(matches) = matches.subcommand_matches("grafana") {
            let url = matches.value_of("GRAFANA_URL").unwrap();
            let key = matches.value_of("GRAFANA_API_KEY").unwrap();
            let datasource_url = matches.value_of("datasource_url").unwrap();
            let query = matches.value_of("query").unwrap();
            let from = matches.value_of("from").unwrap().parse().unwrap();
            let to = matches.value_of("to").unwrap().parse().unwrap();
            let step = matches.value_of("step").unwrap().parse().unwrap();
            return CLI {
                query_config: QueryConfig {
                    datasource_config: DatasourceConfig::Grafana(GrafanaConfig {
                        url: url.to_owned(),
                        api_key: key.to_owned(),
                        datasource_url: datasource_url.to_owned(),
                        query: query.to_owned()
                    }),
                    from,
                    to,
                    step,
                },
            };
        };

        if let Some(matches) = matches.subcommand_matches("prometheus") {
            let url = matches.value_of("PROM_URL").unwrap();
            let query = matches.value_of("query").unwrap();
            let from = matches.value_of("from").unwrap().parse().unwrap();
            let to = matches.value_of("to").unwrap().parse().unwrap();
            let step = matches.value_of("step").unwrap().parse().unwrap();
            return CLI {
                query_config: QueryConfig {
                    datasource_config: DatasourceConfig::Prometheus(PrometheusConfig {
                        url: url.to_owned(),
                        query: query.to_owned()
                    }),
                    from,
                    to,
                    step,
                },
            };
        };

        if let Some(matches) = matches.subcommand_matches("influx") {
            let url = matches.value_of("PROM_URL").unwrap();
            let org_id = matches.value_of("ORG_ID").unwrap();
            let token = matches.value_of("TOKEN").unwrap();
            let query = matches.value_of("query").unwrap();
            let from = matches.value_of("from").unwrap().parse().unwrap();
            let to = matches.value_of("to").unwrap().parse().unwrap();
            let step = matches.value_of("step").unwrap().parse().unwrap();
            return CLI {
                query_config: QueryConfig {
                    datasource_config: DatasourceConfig::Influx(InfluxConfig {
                        url: url.to_owned(),
                        org_id: org_id.to_owned(),
                        token: token.to_owned(),
                        query: query.to_owned()
                    }),
                    from,
                    to,
                    step,
                },
            };
        };

        panic!("Unknown datasource");
    }
}
