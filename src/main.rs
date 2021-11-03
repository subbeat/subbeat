use subbeat::{
    datasources::resolve,
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

    if r.data.keys().len() > 0 {
        let key = r.data.keys().nth(0).unwrap();
        println!("timestamp\t{}", key);
        let vs = &r.data[key];
        for (t, v) in vs.iter() {
            println!("{}\t{}", t, v);
        }
    } else {
        println!("no_data");
    }

    Ok(())
}
