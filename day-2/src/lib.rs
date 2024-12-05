pub mod part_1;
pub mod part_2;

pub(crate) const MIN_DIFF: usize = 1;
pub(crate) const MAX_DIFF: usize = 3;

#[derive(Debug)]
pub(crate) struct Report(Vec<usize>);

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        let levels = value
            .split_whitespace()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self(levels)
    }
}
