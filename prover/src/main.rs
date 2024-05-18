use std::{error::Error, result::Result};
use std::process::{Command};
use std::io::Write;
use clap::Parser;

use gevulot_common::WORKSPACE_PATH;
use gevulot_shim::{Task, TaskResult};

#[derive(Parser, Debug, Default, PartialEq)]
struct Args {
    /// path to the stone-prover executable
    #[arg(long)]
    prover_path: Option<String>,

    /// directory where the private/public input and config params files are located
    #[arg(long)]
    data_dir: Option<String>,

    /// program name without extension, e.g. fibonacci
    #[arg(long)]
    program_name: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("{WORKSPACE_PATH}");
    if args == Args::default() {
        gevulot_shim::run(run_task)
    } else {
        Ok(
            run_prover(
                &args.prover_path.unwrap(), 
                &args.data_dir.unwrap(),
                &args.program_name.unwrap()
            )
            .map(|output_file| println!("stone proof file: {output_file}"))?
        )
    }
}

// The main function that executes the prover program.
fn run_task(task: Task) -> Result<TaskResult, Box<dyn Error>> {
    // Display program arguments we received. These could be used for
    // e.g. parsing CLI arguments with clap.
    println!("prover: task.args: {:?}", &task.args);

    let prover_path = format!("{WORKSPACE_PATH}/{}", task.args[1]);
    let input_dir = format!("{WORKSPACE_PATH}/{}", task.args[2]);
    let program_name = format!("{WORKSPACE_PATH}/{}", task.args[3]);
    // -----------------------------------------------------------------------
    // Here would be the control logic to run the prover with given arguments.
    // -----------------------------------------------------------------------
    let proof_file = run_prover(&prover_path, &input_dir, &program_name)?;

    // Write generated proof to a file.
//    std::fs::write("/workspace/proof.dat", b"this is a proof.")?;

    // Return TaskResult with reference to the generated proof file.
//    task.result(vec![], vec![String::from("/workspace/proof.dat")])
    task.result(vec![], vec![proof_file])
}

fn run_prover(prover_path: &str, input_dir: &str, program_name: &str) -> Result<String, std::io::Error> {
    let out_file = format!("{input_dir}/{program_name}_proof.json");
    let private_input_file = format!("{input_dir}/{program_name}_private_input.json");
    let public_input_file = format!("{input_dir}/{program_name}_public_input.json");
    let prover_config_file = format!("{input_dir}/cpu_air_prover_config.json");
    let cpu_air_params_file = format!("{input_dir}/cpu_air_params.json");
        
    Command::new(prover_path)
        .arg(format!("--out-file={out_file}"))
        .arg(format!("--private_input_file={private_input_file}"))
        .arg(format!("--public_input_file={public_input_file}"))
        .arg(format!("--prover_config_file={prover_config_file}"))
        .arg(format!("--parameter_file={cpu_air_params_file}"))
        .stdout(std::process::Stdio::inherit())
        .output()
        .and_then(|output| output
            .status
            .success()
            .then_some(out_file.to_owned())
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
