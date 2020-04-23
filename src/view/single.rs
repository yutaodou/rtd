use std::io::Write;

use crate::task::Task;

pub fn render<W: Write>(task: &Task, w: &mut W) -> Result<(), String> {
    let done = if task.done { "x" } else { "-" };
    let result = format!(
        "[{}] {}. {} :{} +{}{}",
        done,
        task.id,
        task.title,
        task.list,
        task.priority.to_string(),
        task.due_date.map_or("".to_string(), |date| format!(" @{}", date.format("%F"))),
    );
    writeln!(w, "{}", result).unwrap();
    Ok(())
}
