use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match &args[1][..] {
            "planning" => {
                let agenda = agenda::types_planning();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            _ => {}
        }
    }

    eprintln!("Usage: types (planning)");

    Ok(())
}
