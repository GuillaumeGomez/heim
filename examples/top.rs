use std::time::Duration;
use std::usize;

use futures::prelude::*;
use heim::{
    process::{self, Process, ProcessResult},
    units::{ratio, Ratio},
};

async fn usage(process: Process) -> ProcessResult<(process::Process, Ratio)> {
    let usage_1 = process.cpu_usage().await?;
    futures_timer::Delay::new(Duration::from_millis(100)).await;
    let usage_2 = process.cpu_usage().await?;

    Ok((process, usage_2 - usage_1))
}

#[tokio::main]
async fn main() -> ProcessResult<()> {
    let processes = process::processes()
        .map_ok(|process| {
            // Note that there is no `.await` here,
            // as we want to pass the returned future
            // into the `.try_buffer_unordered`.
            usage(process)
        })
        .try_buffer_unordered(usize::MAX);
    tokio::pin!(processes);

    println!("| {:6} | {:40} | {:4} % |", "pid", "name", "CPU");
    while let Some(res) = processes.next().await {
        let (process, usage) = res?;

        println!(
            "| {:6} | {:40} | {:.2} |",
            process.pid(),
            process.name().await?,
            usage.get::<ratio::percent>()
        );
    }

    Ok(())
}
