use crate::model::Task;
use prettytable::{Table, format, Row};
use crate::view::formatter::Formatter;

pub fn render(task: &Task) -> Result<(), String> {
    let mut table = Table::new();
    let formatter = Formatter::new(task.clone());

    let mut row = Row::empty();
    row.add_cell(cell!(formatter.task_id()));
    if task.done { row.add_cell(cell!(formatter.done())) };
    row.add_cell(cell!(formatter.title(None)));
    row.add_cell(cell!(formatter.priority()));
    row.add_cell(cell!(formatter.due_date()));
    row.add_cell(cell!(formatter.task_list(false)));

    table.add_row(row);

    let format = format::FormatBuilder::new().padding(1, 1).build();
    table.set_format(format);
    table.printstd();

    Ok(())
}
