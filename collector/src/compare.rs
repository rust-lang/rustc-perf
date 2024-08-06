use std::sync::Arc;

use database::{
    metric::Metric,
    selector::{BenchmarkQuery, CompileBenchmarkQuery},
    ArtifactId, Connection,
};
use tabled::{Table, Tabled};

/// Compare 2 artifacts and print the result.
pub async fn compare_artifacts(
    mut conn: Box<dyn Connection>,
    base: String,
    modified: String,
) -> anyhow::Result<()> {
    let index = database::Index::load(&mut *conn).await;

    let query = CompileBenchmarkQuery::default()
        .metric(database::selector::Selector::One(Metric::InstructionsUser));
    let resp = query
        .execute(
            &mut *conn,
            &index,
            Arc::new(vec![ArtifactId::Tag(base), ArtifactId::Tag(modified)]),
        )
        .await
        .unwrap();

    let tuple_pstats = resp
        .into_iter()
        .map(|resp| {
            let points = resp.series.points.collect::<Vec<_>>();
            (points[0], points[1])
        })
        .collect::<Vec<_>>();

    #[derive(Tabled)]
    struct Regression {
        count: usize,
        #[tabled(display_with = "display_range")]
        range: (Option<f64>, Option<f64>),
        #[tabled(display_with = "display_mean")]
        mean: Option<f64>,
    }

    fn format_value(value: Option<f64>) -> String {
        match value {
            Some(value) => format!("{:+.2}%", value),
            None => "-".to_string(),
        }
    }

    fn display_range(&(min, max): &(Option<f64>, Option<f64>)) -> String {
        format!("[{}, {}]", &format_value(min), &format_value(max))
    }

    fn display_mean(value: &Option<f64>) -> String {
        match value {
            Some(value) => format!("{:+.2}%", value),
            None => "-".to_string(),
        }
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

    let change = tuple_pstats
        .iter()
        .filter_map(|&(a, b)| match (a, b) {
            (Some(a), Some(b)) => {
                if a == 0.0 {
                    None
                } else {
                    Some((b - a) / a)
                }
            }
            (_, _) => None,
        })
        .collect::<Vec<_>>();
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

    #[derive(Tabled)]
    struct NamedRegression {
        name: String,
        #[tabled(inline)]
        regression: Regression,
    }

    let regressions = [negative_change, positive_change, change]
        .into_iter()
        .map(|c| Regression::from(&c))
        .zip(["❌", "✅", "✅, ❌"])
        .map(|(c, label)| NamedRegression {
            name: label.to_string(),
            regression: c,
        })
        .collect::<Vec<_>>();

    println!("{}", Table::new(regressions));

    Ok(())
}
