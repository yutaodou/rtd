use std::io::Write;

use crate::task::Task;

pub fn render<W: Write>(task: &Task, w: &mut W) -> Result<(), &'static str> {
    let done = if task.done { "x" } else { "-" };
    let result = format!(
        "[{}] {} {} ~{} !{}",
        done,
        task.id,
        task.title,
        task.list,
        task.priority.to_string()
    );
    writeln!(w, "{}", result).unwrap();
    Ok(())
}