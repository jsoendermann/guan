mod commands;
mod pipeline;

use clap::{App, Arg, SubCommand};

use commands::command::GuanCommand;
use commands::deploy::{DeployArgs, DeployCommand};
use commands::ls_stages::{LsStagesArgs, LsStagesCommand};
use commands::run_stage::{RunStageArgs, RunStageCommand};

fn main() {
    let pipeline_file_path_arg = Arg::with_name("pipeline_file_path")
        .short("p")
        .long("pipeline")
        .default_value("pipeline.yml")
        .help("The file path to the pipeline definition file.");
    let workdir_arg = Arg::with_name("workdir")
        .short("w")
        .long("workdir")
        .default_value(".")
        .help("The directory in which the pipeline commands are run.");
    let matches = App::new("Guan")
        .version("1.0")
        .author("Jan Soendermann")
        .about("Deployment pipelines for your personal projects.")
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploy your code by executing a pipeline.")
                .arg(&pipeline_file_path_arg)
                .arg(&workdir_arg),
        )
        .subcommand(
            SubCommand::with_name("run-stage")
                .about("Runs an individual stage of your pipeline.")
                .arg(&pipeline_file_path_arg)
                .arg(&workdir_arg)
                .arg(
                    Arg::with_name("stage_name")
                        .required(true)
                        .help("The id of the stage to be executed."),
                ),
        )
        .subcommand(
            SubCommand::with_name("ls-stages")
                .about("List all stages of your pipeline.")
                .arg(&pipeline_file_path_arg),
        )
        .get_matches();

    let command: Box<dyn GuanCommand> = if let Some(deploy) = matches.subcommand_matches("deploy") {
        let pipeline_file_path = deploy.value_of("pipeline_file_path").unwrap().to_string();
        let workdir = deploy.value_of("workdir").unwrap().to_string();
        let args = DeployArgs {
            pipeline_file_path,
            workdir,
        };

        Box::new(DeployCommand::new(args))
    } else if let Some(run_stage) = matches.subcommand_matches("run-stage") {
        let stage_name = run_stage.value_of("stage_name").unwrap().to_string();
        let pipeline_file_path = run_stage
            .value_of("pipeline_file_path")
            .unwrap()
            .to_string();
        let workdir = run_stage.value_of("workdir").unwrap().to_string();

        let args = RunStageArgs {
            stage_name,
            pipeline_file_path,
            workdir,
        };

        Box::new(RunStageCommand::new(args))
    } else if let Some(ls_stage) = matches.subcommand_matches("ls-stages") {
        let pipeline_file_path = ls_stage.value_of("pipeline_file_path").unwrap().to_string();

        let args = LsStagesArgs { pipeline_file_path };

        Box::new(LsStagesCommand::new(args))
    } else {
        panic!("No subcommand selected");
    };

    command.execute().unwrap();
}
