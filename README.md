# Guan

Add deployment pipelines to your personal projects. `guan` executes `pipeline.yml` files you add to your projects. To deploy your project, you simply run `guan deploy` and the stages in your pipeline get executed. Think Github actions running on your local machine.

### When should I use this?

When you're working on a project as a single developer and don't need CI. `guan` also lets you debug your pipelines much more easily since everything is executed locally.

Some additional features that might make you choose `guan` over a bash script:

- run individual stages of your pipeline easily for those emergency prod deployments.
- define dependencies between your pipeline stages and `guan` automatically parallelises execution. (to be implemented)
- a nice CLI. (to be implemented)
- estimates for time remaining as your pipeline is being executed. (to be implemented)

### When shouldn't I use this?

When you have more than one person working on your codebase.

## Todo

- dependencies and parallel executions for stages
- env vars
- multi command stages
- nicer console output https://docs.rs/indicatif/0.15.0/indicatif/
- logfiles
- stream output
- runtime estimates

## commands

- show-dependencies
