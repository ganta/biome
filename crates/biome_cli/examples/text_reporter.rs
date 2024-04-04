use biome_cli::{Execution, Reporter, ReporterVisitor, TraversalSummary};
use std::time::Duration;

/// This will be the visitor, which where we **write** the data
struct BufferVisitor(String);

/// This is the reporter, which will be a type that will hold the information needed to the reporter
struct TextReport {
    summary: TraversalSummary,
}

impl Reporter for TextReport {
    fn write(&mut self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        let execution = Execution::new_format();
        visitor.report_summary(&execution, &self.summary)?;
        Ok(())
    }
}

impl ReporterVisitor for BufferVisitor {
    fn report_summary(
        &mut self,
        execution: &Execution,
        summary: &TraversalSummary,
    ) -> std::io::Result<()> {
        self.report_total(
            execution,
            summary.changed() + summary.changed(),
            summary.duration(),
        )
    }

    fn report_total(
        &mut self,
        __execution: &Execution,
        total: usize,
        _duration: Duration,
    ) -> std::io::Result<()> {
        self.0.push_str(&format!("Total is {}", total));
        Ok(())
    }
}

pub fn main() {
    let summary = TraversalSummary::default()
        .with_changed(32)
        .with_unchanged(28);
    let mut visitor = BufferVisitor(String::new());
    let mut reporter = TextReport { summary };
    reporter.write(&mut visitor).unwrap();

    assert_eq!(visitor.0.as_str(), "Total is 64")
}
