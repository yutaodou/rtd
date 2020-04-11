use std::io::Write;

use ansi_term::Style;

use crate::task::Task;

pub struct Render {
    pub tasks: Vec<Task>,
}

impl Render {
    pub fn render<W: Write>(self: &Self, w: &mut W) -> Result<(), &'static str> {
        let pending_tasks: Vec<&Task> = self.tasks.iter().filter(|task| task.done == false).collect();
        let mut lists: Vec<&str> = pending_tasks.iter().map(|task| task.list.as_str()).collect();
        lists.dedup();
        for list in lists.iter() {
            writeln!(w, "{}", list).unwrap();

            let list_content = pending_tasks.iter().filter(|task| task.list == *list);
            for task in list_content {
                self.render_single(w, task);
            }
        }
        Ok(())
    }

    fn render_single<W: Write>(self: &Self, w: &mut W, task: &Task) {
        let title = if task.done {
            Style::new().strikethrough().paint(&task.title)
        } else {
            Style::default().paint(&task.title)
        };

        writeln!(w, "{:>4}. {} !{}", task.id, title, Style::default().paint(task.priority.to_string())).unwrap();
    }
}
