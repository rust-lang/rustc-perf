use anyhow::Context;
use database::selector::{BenchmarkQuery, CompileBenchmarkQuery, CompileTestCase};
use database::{metric::Metric, ArtifactId, Commit, Connection, Index};
use ratatui::prelude::Stylize;
use ratatui::widgets::{Cell, List, ListState, Paragraph, Row, Table, TableState};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
    widgets::Block,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tabled::Tabled;

static ALL_METRICS: &[Metric] = &[
    Metric::InstructionsUser,
    Metric::CyclesUser,
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
    Metric::Cycles,
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
        commit
            .map(|commit| {
                aids.iter()
                    .find(|c| c.sha == commit)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("{label} commit {commit} not found"))
            })
            .transpose()
    }

    let base: Option<Commit> = check_commit(&aids, base, "Base")?;
    if let Some(ref base) = base {
        aids.retain(|c| c.sha != base.sha);
    }
    let modified: Option<Commit> = check_commit(&aids, modified, "Modified")?;

    let mut terminal = ratatui::init();

    let db_state = DbState { index, db: conn };
    let mut screen: Box<dyn Screen> = match (base, modified) {
        (Some(base), Some(modified)) => {
            Box::new(CompareScreen::new(db_state, metric, base, modified).await?)
        }
        (Some(base), None) => next_selection_screen(db_state, metric, base, aids).await?,
        (None, None) => Box::new(SelectArtifactScreen::new(
            db_state,
            metric,
            aids,
            SelectState::SelectingBase,
        )),
        (None, Some(_)) => {
            return Err(anyhow::anyhow!(
                "If modified commit is pre-selected, base commit must also be pre-selected"
            ));
        }
    };

    loop {
        terminal.draw(|frame| {
            screen.draw(frame);
        })?;
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                key => {
                    if let Some(action) = screen.handle_key(key).await? {
                        match action {
                            Action::SelectArtifacts(next_screen) => {
                                screen = next_screen;
                            }
                        }
                    }
                }
            }
        }
    }
    ratatui::restore();

    Ok(())
}

struct DbState {
    index: Index,
    db: Box<dyn Connection>,
}

enum Action {
    SelectArtifacts(Box<dyn Screen>),
}

trait Screen {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_key(
        &mut self,
        key: KeyCode,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Action>>> + '_>>;
}

enum SelectState {
    SelectingBase,
    SelectingModified { base: Commit },
}

struct SelectArtifactScreen {
    aids: Vec<Commit>,
    select_state: SelectState,
    list_state: ListState,
    db_state: Option<DbState>,
    metric: Metric,
}

impl SelectArtifactScreen {
    fn new(
        db_state: DbState,
        metric: Metric,
        aids: Vec<Commit>,
        select_state: SelectState,
    ) -> Self {
        Self {
            aids,
            select_state,
            list_state: ListState::default().with_selected(Some(0)),
            db_state: Some(db_state),
            metric,
        }
    }
}

impl Screen for SelectArtifactScreen {
    fn draw(&mut self, frame: &mut Frame) {
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

    fn handle_key(
        &mut self,
        key: KeyCode,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Action>>> + '_>> {
        Box::pin(async move {
            match key {
                KeyCode::Down => self.list_state.select_next(),
                KeyCode::Up => self.list_state.select_previous(),
                KeyCode::Enter => {
                    let mut aids = self.aids.clone();
                    let index = self.list_state.selected().unwrap();
                    let selected = aids.remove(index);

                    let next_screen: Box<dyn Screen> = match &self.select_state {
                        SelectState::SelectingBase => {
                            next_selection_screen(
                                self.db_state.take().unwrap(),
                                self.metric,
                                selected,
                                aids,
                            )
                            .await?
                        }
                        SelectState::SelectingModified { base } => Box::new(
                            CompareScreen::new(
                                self.db_state.take().unwrap(),
                                self.metric,
                                base.clone(),
                                selected,
                            )
                            .await?,
                        ),
                    };
                    return Ok(Some(Action::SelectArtifacts(next_screen)));
                }
                _ => {}
            };
            Ok(None)
        })
    }
}

/// Directly goes to comparison if there is only a single artifact ID left, otherwise
/// opens the selection screen for the modified artifact.
async fn next_selection_screen(
    db_state: DbState,
    metric: Metric,
    base: Commit,
    aids: Vec<Commit>,
) -> anyhow::Result<Box<dyn Screen>> {
    let screen = match aids.as_slice() {
        [commit] => Box::new(CompareScreen::new(db_state, metric, base, commit.clone()).await?)
            as Box<dyn Screen>,
        _ => Box::new(SelectArtifactScreen::new(
            db_state,
            metric,
            aids,
            SelectState::SelectingModified { base },
        )),
    };
    Ok(screen)
}

struct TestCaseMeasurement {
    case: CompileTestCase,
    before: Option<f64>,
    after: Option<f64>,
}

impl TestCaseMeasurement {
    fn change(&self) -> Option<f64> {
        match (self.before, self.after) {
            (Some(a), Some(b)) => {
                if a == 0.0 {
                    None
                } else {
                    Some((b - a) / a * 100.0)
                }
            }
            (_, _) => None,
        }
    }
}

struct CompareScreen {
    base: Commit,
    modified: Commit,
    data: Vec<TestCaseMeasurement>,
    metric: Metric,
    db_state: DbState,
    table_state: TableState,
}

impl CompareScreen {
    async fn new(
        mut db_state: DbState,
        metric: Metric,
        base: Commit,
        modified: Commit,
    ) -> anyhow::Result<Self> {
        let data = load_data(
            metric,
            &db_state.index,
            db_state.db.as_mut(),
            &base,
            &modified,
        )
        .await?;
        Ok(Self {
            base,
            modified,
            db_state,
            metric,
            data,
            table_state: TableState::default(),
        })
    }

    async fn reload_data(&mut self) -> anyhow::Result<()> {
        self.data = load_data(
            self.metric,
            &self.db_state.index,
            self.db_state.db.as_mut(),
            &self.base,
            &self.modified,
        )
        .await?;
        Ok(())
    }
}

impl Screen for CompareScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let summary_table = draw_summary_table(&self.data);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    // +2 because of borders
                    Constraint::Min((summary_table.lines().count() + 2) as u16),
                    Constraint::Length(2),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(frame.area());

        frame.render_widget(
            Paragraph::new(Text::raw(summary_table)).block(Block::bordered().title("Summary")),
            layout[0],
        );

        render_metric(frame, self.metric, layout[1]);

        let header = Row::new(vec![
            Line::from("Benchmark"),
            Line::from("Profile"),
            Line::from("Scenario"),
            Line::from("Backend"),
            Line::from("Change").right_aligned(),
        ])
        .style(Style::default().bold());
        let widths = [
            Constraint::Percentage(25),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
            Constraint::Percentage(10),
            Constraint::Percentage(30),
        ];

        let mut rows = self
            .data
            .iter()
            .filter_map(|row| row.change().map(|change| (row, change)))
            .collect::<Vec<_>>();

        rows.sort_by(|(_, a), (_, b)| {
            a.abs()
                .partial_cmp(&b.abs())
                .expect("cannot compare two results")
                .reverse()
        });

        let table = Table::new(
            rows.into_iter()
                .map(|(measurement, change)| {
                    let benchmark = Cell::from(measurement.case.benchmark.to_string());
                    let profile = Cell::from(measurement.case.profile.to_string());
                    let scenario = Cell::from(measurement.case.scenario.to_string());
                    let backend = Cell::from(measurement.case.backend.to_string());
                    let change: Cell = {
                        let span = Line::from(format!("{change:+.2}%"));
                        if change < 0.0 {
                            span.green()
                        } else {
                            span.red()
                        }
                        .right_aligned()
                        .into()
                    };

                    Row::new([benchmark, profile, scenario, backend, change])
                })
                .collect::<Vec<Row>>(),
            widths,
        )
        .header(header)
        .column_spacing(1)
        .highlight_symbol("> ")
        .block(Block::bordered().title("Measurements"))
        .row_highlight_style(Style::new().bold());

        let table_layout =
            Layout::new(Direction::Horizontal, [Constraint::Max(120)]).split(layout[2]);
        frame.render_stateful_widget(table, table_layout[0], &mut self.table_state);
    }

    fn handle_key(
        &mut self,
        key: KeyCode,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Option<Action>>> + '_>> {
        Box::pin(async move {
            match key {
                KeyCode::Down => self.table_state.select_next(),
                KeyCode::Up => self.table_state.select_previous(),
                KeyCode::Char('a') => {
                    self.metric = select_metric(self.metric, -1);
                    self.reload_data().await?;
                }
                KeyCode::Char('s') => {
                    self.metric = select_metric(self.metric, 1);
                    self.reload_data().await?;
                }
                _ => {}
            }

            Ok(None)
        })
    }
}

fn select_metric(current: Metric, direction: isize) -> Metric {
    let index = ALL_METRICS.iter().position(|m| *m == current).unwrap_or(0) as isize;
    let index = ((index + direction) + ALL_METRICS.len() as isize) % ALL_METRICS.len() as isize;
    ALL_METRICS[index as usize]
}

fn render_metric(frame: &mut Frame, metric: Metric, area: Rect) {
    frame.render_widget(
        Line::from(vec![
            "Metric: ".into(),
            metric.as_str().bold(),
            " (switch: A/S)".into(),
        ]),
        area,
    )
}

async fn load_data(
    metric: Metric,
    index: &Index,
    conn: &mut dyn Connection,
    base: &Commit,
    modified: &Commit,
) -> anyhow::Result<Vec<TestCaseMeasurement>> {
    let query = CompileBenchmarkQuery::default().metric(database::selector::Selector::One(metric));
    let resp = query
        .execute(
            conn,
            index,
            Arc::new(vec![
                ArtifactId::Commit(base.clone()),
                ArtifactId::Commit(modified.clone()),
            ]),
        )
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))
        .context("Cannot load data from the DB")?;

    Ok(resp
        .into_iter()
        .map(|resp| {
            let points = resp.series.points.clone().collect::<Vec<_>>();
            TestCaseMeasurement {
                case: resp.test_case,
                before: points[0],
                after: points[1],
            }
        })
        .collect::<Vec<_>>())
}

fn calculate_changes(pstats: &[TestCaseMeasurement]) -> Vec<f64> {
    pstats
        .iter()
        .filter_map(|measurement| measurement.change())
        .collect::<Vec<f64>>()
}

fn draw_summary_table(pstats: &[TestCaseMeasurement]) -> String {
    let change = calculate_changes(pstats);
    let negative_change = change
        .iter()
        .copied()
        .filter(|&c| c < 0.0)
        .collect::<Vec<_>>();
    let positive_change = change
        .iter()
        .copied()
        .filter(|&c| c > 0.0)
        .collect::<Vec<_>>();

    #[derive(Tabled, Debug)]
    struct NamedRegression {
        name: String,
        #[tabled(inline)]
        regression: Regression,
    }

    let results = [positive_change, negative_change, change]
        .into_iter()
        .map(|c| Regression::from(&c))
        .zip(["❌", "✅", "✅, ❌"])
        .map(|(c, label)| NamedRegression {
            name: label.to_string(),
            regression: c,
        })
        .collect::<Vec<_>>();

    tabled::Table::new(results).to_string()
}

#[derive(Tabled, Debug)]
struct Regression {
    count: usize,
    #[tabled(display("display_range"))]
    range: (Option<f64>, Option<f64>),
    #[tabled(display("format_value"))]
    mean: Option<f64>,
}

fn format_value(value: &Option<f64>) -> String {
    match value {
        Some(value) => format!("{value:+.2}%"),
        None => "-".to_string(),
    }
}

fn display_range(&(min, max): &(Option<f64>, Option<f64>)) -> String {
    format!("[{}, {}]", &format_value(&min), &format_value(&max))
}

impl From<&Vec<f64>> for Regression {
    fn from(value: &Vec<f64>) -> Self {
        let min = value.iter().copied().min_by(|a, b| a.total_cmp(b));
        let max = value.iter().copied().max_by(|a, b| a.total_cmp(b));
        let count = value.len();

        Regression {
            range: (min, max),
            count,
            mean: if count == 0 {
                None
            } else {
                Some(value.iter().sum::<f64>() / count as f64)
            },
        }
    }
}
