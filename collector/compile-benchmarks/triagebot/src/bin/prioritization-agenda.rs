use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let agenda = agenda::prioritization();

    print!("{}", agenda.call().await?);
    Ok(())
}
