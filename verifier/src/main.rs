use std::{error::Error, result::Result};
use std::process::{Command};
use std::io::Write;
use clap::Parser;

use gevulot_common::WORKSPACE_PATH;
use gevulot_shim::{Task, TaskResult};

#[derive(Parser, Debug, Default, PartialEq)]
struct Args {
    /// path to the stone-verifier executable
    #[arg(long)]
    verifier_path: Option<String>,

    /// path to proof file
    #[arg(long)]
    in_file: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("{WORKSPACE_PATH}");
    if args == Args::default() {
        gevulot_shim::run(run_task)
    } else {
        Ok(run_verifier(
            &args.verifier_path.unwrap(), 
            &args.in_file.unwrap(),
        )?)
    }
}

// The main function that executes the prover program.
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    // Display program arguments we received. These could be used for
    // e.g. parsing CLI arguments with clap.
    println!("verifier: task.args: {:?}", &task.args);

    let verifier_path = format!("{WORKSPACE_PATH}/{}", task.args[1]);
    let in_file = format!("{WORKSPACE_PATH}/{}", task.args[2]);
    // -----------------------------------------------------------------------
    // Here would be the control logic to run the prover with given arguments.
    // -----------------------------------------------------------------------
    run_verifier(&verifier_path, &in_file,)?;

    // Write generated proof to a file.
//    std::fs::write("/workspace/proof.dat", b"this is a proof.")?;

    // Return TaskResult with reference to the generated proof file.
//    task.result(vec![], vec![String::from("/workspace/proof.dat")])
    task.result(vec![], vec![])
}

fn run_verifier(verifier_path: &str, in_file: &str) -> Result<(), std::io::Error> {
    Command::new(verifier_path)
        .arg(format!("--in-file={in_file}"))
        .stdout(std::process::Stdio::inherit())
        .output()
        .and_then(|output| output
            .status
            .success()
            .then_some(())
            .ok_or({
                let _ = std::io::stdout().write_all(&output.stdout);
                let _ = std::io::stdout().write_all(&output.stderr);

                std::io::Error::new(
                    std::io::ErrorKind::Other, 
                    format!("error, status: {:?}", output.status.code())
                )
            })
        )
}
