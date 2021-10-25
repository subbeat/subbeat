use subbeat::{datasources::grafana, metric::Metric};

mod cli;
mod types;

#[tokio::main]
async fn main() -> types::Result<()> {
    let cli = cli::CLI::new();

    let gs = grafana::Grafana::new(
        cli.query_config.url.to_string(),
        cli.query_config.key.to_string(),
        cli.query_config.datasource_url.to_string(),
        cli.query_config.query.to_string(),
    );

    let r = gs
        .query(
            cli.query_config.from,
            cli.query_config.to,
            cli.query_config.step,
        )
        .await?;

    let key = r.data.keys().nth(0).unwrap();
    println!("{}", key);

    let vs = &r.data[key];
    for (t, v) in vs.iter() {
        println!("{}\t{}", t, v);
    }

    Ok(())
}
