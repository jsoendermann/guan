mod commands;
mod pipeline;

use clap::{App, Arg, SubCommand};

use commands::command::GuanCommand;
use commands::deploy::{DeployArgs, DeployCommand};

fn main() {
    let matches = App::new("Guan")
        .version("1.0")
        .author("Jan Soendermann")
        .about("Pipelines for your personal projects.")
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploy your code by executing a pipeline.")
                .arg(
                    Arg::with_name("pipeline_file_path")
                        .short("p")
                        .long("pipeline")
                        .default_value("pipeline.yml")
                        .help("The file containing the pipeline definition."),
                )
                .arg(
                    Arg::with_name("workdir")
                        .short("d")
                        .long("workdir")
                        .default_value(".")
                        .help("The directory in which the pipeline commands are run."),
                ),
        )
        .get_matches();

    let command = if let Some(deploy) = matches.subcommand_matches("deploy") {
        let args = DeployArgs {
            pipeline_file_path: deploy.value_of("pipeline_file_path").unwrap().to_string(),
            workdir: deploy.value_of("workdir").unwrap().to_string(),
        };

        DeployCommand::new(args)
    } else {
        panic!("No subcommand selected");
    };

    command.execute().unwrap();
}
