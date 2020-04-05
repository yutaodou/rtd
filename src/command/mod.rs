mod add;
mod cmd;

pub use add::Add;

pub trait Command {
    fn exec(self: &Self) -> Result<(), &'static str>;
}

// pub fn parse<'a>(args: &'a [String]) -> Box<dyn Command> {
//     let cmd = Add::new(args).unwrap();
//     Box::new(cmd)
// }
