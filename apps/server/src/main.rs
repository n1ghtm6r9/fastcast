use std::process::ExitCode;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> ExitCode {
    match app::app::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fastcast stopped: {}", error);
            ExitCode::FAILURE
        }
    }
}
