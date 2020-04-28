pub trait MultiProcessor<T> {
    fn process_single(&self, task_id: u32) -> Result<T, String>;

    fn process(&self, inputs: Vec<&str>, outputter: Box<dyn Fn(T) -> ()>) -> Result<(), String> {
        let results = inputs.iter().map(|input| {
            input
                .parse()
                .map_err(|_| format!("Invalid task id: {}", input))
                .and_then(|task_id| self.process_single(task_id))
        });

        let mut captured_error = None;
        results.for_each(|result| match result {
            Ok(value) => outputter(value),
            Err(err) => captured_error = Some(err),
        });

        match captured_error {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }
}
