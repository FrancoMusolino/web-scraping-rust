use std::{env, path::PathBuf, process, time::Instant};
use web_scraping_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let mut file_path = PathBuf::from(cwd);
    file_path.push("dolar.svg");

    println!("Generando archivo...");

    let start = Instant::now();

    if let Err(err) = run().await {
        eprintln!("Error running the program: {err}");
        process::exit(1);
    }

    let duration = start.elapsed();
    println!("Archivo generado en: {}ms", duration.as_millis());
    println!("{}", file_path.display());

    Ok(())
}
