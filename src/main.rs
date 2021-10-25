use subbeat::{
    datasources::{grafana, resolve},
    metric::Metric,
};

mod cli;
mod types;

#[tokio::main]
async fn main() -> types::Result<()> {
    let cli = cli::CLI::new();

    let ds = resolve(&cli.query_config);

    let r = ds
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
