use std::io::Write;

use crate::model::Task;

pub fn render<W: Write>(task: &Task, w: &mut W) -> Result<(), String> {
    let done = if task.done { "x" } else { "-" };
    let result = format!(
        "[{}] {}. {} :{} +{}{}",
        done,
        task.id,
        task.title,
        task.list,
        task.priority.to_string(),
        task.due_date
            .as_ref()
            .map_or("".to_string(), |date| format!(" @{}", date)),
    );
    writeln!(w, "{}", result).unwrap();
    Ok(())
}
