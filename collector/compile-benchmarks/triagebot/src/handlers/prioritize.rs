use crate::{
    config::PrioritizeConfig,
    github::{self, Event},
    handlers::Context,
};
use parser::command::prioritize::PrioritizeCommand;

pub(super) async fn handle_command(
    ctx: &Context,
    config: &PrioritizeConfig,
    event: &Event,
    _: PrioritizeCommand,
) -> anyhow::Result<()> {
    let mut labels = vec![];
    labels.push(github::Label {
        name: config.label.to_owned(),
    });
    event
        .issue()
        .unwrap()
        .add_labels(&ctx.github, labels)
        .await?;
    Ok(())
}
