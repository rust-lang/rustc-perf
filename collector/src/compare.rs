use database::{metric::Metric, Commit, Connection, Index};
use ratatui::widgets::{List, ListState};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::Block,
};

static ALL_METRICS: &[Metric] = &[
    Metric::InstructionsUser,
    Metric::Cycles,
    Metric::WallTime,
    Metric::MaxRSS,
    Metric::LinkedArtifactSize,
    Metric::AssemblyFileSize,
    Metric::BranchMisses,
    Metric::CacheMisses,
    Metric::CodegenUnitLlvmIrCount,
    Metric::CodegenUnitSize,
    Metric::ContextSwitches,
    Metric::CpuClock,
    Metric::CpuClockUser,
    Metric::CrateMetadataSize,
    Metric::CyclesUser,
    Metric::DepGraphSize,
    Metric::DocByteSize,
    Metric::DwoFileSize,
    Metric::Faults,
    Metric::FaultsUser,
    Metric::LlvmBitcodeSize,
    Metric::LlvmIrSize,
    Metric::ObjectFileSize,
    Metric::QueryCacheSize,
    Metric::TaskClock,
    Metric::TaskClockUser,
    Metric::WorkProductIndexSize,
];

/// Compare 2 artifacts and print the result.
pub async fn compare_artifacts(
    mut conn: Box<dyn Connection>,
    metric: Option<Metric>,
    base: Option<String>,
    modified: Option<String>,
) -> anyhow::Result<()> {
    let index = database::Index::load(&mut *conn).await;

    let metric = metric.unwrap_or(Metric::InstructionsUser);

    let mut aids = index.commits();
    if aids.len() < 2 {
        return Err(anyhow::anyhow!(
            "There are not enough artifacts to compare, at least two are needed"
        ));
    }

    // Error if the selected base/modified commits were not found
    fn check_commit(
        aids: &[Commit],
        commit: Option<String>,
        label: &str,
    ) -> anyhow::Result<Option<Commit>> {
        Ok(commit
            .map(|commit| {
                aids.iter()
                    .find(|c| c.sha == commit)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("{label} commit {commit} not found"))
            })
            .transpose()?)
    }

    let base: Option<Commit> = check_commit(&aids, base, "Base")?;
    if let Some(ref base) = base {
        aids.retain(|c| c.sha != base.sha);
    }
    let modified: Option<Commit> = check_commit(&aids, modified, "Modified")?;

    let mut terminal = ratatui::init();

    let mut screen: Box<dyn Screen> = match (base, modified) {
        (Some(base), Some(modified)) => Box::new(CompareScreen { base, modified }),
        (Some(base), None) => next_selection_screen(base, aids),
        (None, None) => Box::new(SelectArtifactScreen::new(aids, SelectState::SelectingBase)),
        (None, Some(_)) => {
            return Err(anyhow::anyhow!(
                "If modified commit is pre-selected, base commit must also be pre-selected"
            ));
        }
    };
    let mut state = State { metric, index };

    loop {
        terminal.draw(|frame| {
            screen.draw(frame, &state);
        })?;
        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                key => {
                    if let Some(action) = screen.handle_key(key, &mut state) {
                        match action {
                            Action::ChangeScreen(next_screen) => {
                                screen = next_screen;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    ratatui::restore();

    Ok(())
}

struct State {
    metric: Metric,
    index: Index,
}

enum Action {
    ChangeScreen(Box<dyn Screen>),
}

trait Screen {
    fn draw(&mut self, frame: &mut Frame, state: &State);
    fn handle_key(&mut self, key: KeyCode, state: &mut State) -> Option<Action>;
}

enum SelectState {
    SelectingBase,
    SelectingModified { base: Commit },
}

struct SelectArtifactScreen {
    aids: Vec<Commit>,
    select_state: SelectState,
    list_state: ListState,
}

impl SelectArtifactScreen {
    fn new(aids: Vec<Commit>, select_state: SelectState) -> Self {
        Self {
            aids,
            select_state,
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl Screen for SelectArtifactScreen {
    fn draw(&mut self, frame: &mut Frame, _state: &State) {
        let items = self
            .aids
            .iter()
            .map(|commit| format!("{} ({})", commit.sha, commit.date))
            .collect::<Vec<_>>();
        let list = List::new(items)
            .block(Block::bordered().title(format!(
                "Select {} artifact",
                if matches!(self.select_state, SelectState::SelectingBase) {
                    "base"
                } else {
                    "modified"
                }
            )))
            .style(Style::new().white())
            .highlight_style(Style::new().bold())
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }

    fn handle_key(&mut self, key: KeyCode, _state: &mut State) -> Option<Action> {
        match key {
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Enter => {
                let mut aids = self.aids.clone();
                let index = self.list_state.selected().unwrap();
                let selected = aids.remove(index);

                let next_screen: Box<dyn Screen> = match &self.select_state {
                    SelectState::SelectingBase => next_selection_screen(selected, aids),
                    SelectState::SelectingModified { base } => Box::new(CompareScreen {
                        base: base.clone(),
                        modified: selected,
                    }),
                };
                return Some(Action::ChangeScreen(next_screen));
            }
            _ => {}
        };
        None
    }
}

/// Directly goes to comparison if there is only a single artifact ID left, otherwise
/// opens the selection screen for the modified artifact.
fn next_selection_screen(base: Commit, aids: Vec<Commit>) -> Box<dyn Screen> {
    match aids.as_slice() {
        [commit] => Box::new(CompareScreen {
            base,
            modified: commit.clone(),
        }),
        _ => Box::new(SelectArtifactScreen::new(
            aids,
            SelectState::SelectingModified { base },
        )),
    }
}

struct CompareScreen {
    base: Commit,
    modified: Commit,
}

impl Screen for CompareScreen {
    fn draw(&mut self, frame: &mut Frame, state: &State) {
        todo!()
    }

    fn handle_key(&mut self, key: KeyCode, state: &mut State) -> Option<Action> {
        todo!()
    }
}
