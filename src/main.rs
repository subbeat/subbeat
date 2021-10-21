
use subbeat::grafana_service;

mod types;
mod cli;

#[tokio::main]
async fn main() -> types::Result<()> {
    
    let cli = cli::CLI::new();

    let gs = grafana_service::GrafanaService::new(cli.url.to_string(), cli.key.to_string());

    // gs.test_connection().await?;
    // gs.get_datasources().await?;
    // "http://localhost:3000/d/YeBxHjzWz/starter-app-stats?editPanel=2&orgId=1"
    let r = gs
        .extract_metrics(
            &cli.datasource_url,
            &cli.query,
            cli.from,
            cli.to,
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
