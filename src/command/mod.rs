mod add;
mod cmd;
mod done;
mod list;

pub use add::Add;
pub use done::Done;
pub use list::List;

pub trait Command {
    fn exec(self: &Self) -> Result<(), &'static str>;
}

// pub fn parse<'a>(args: &'a [String]) -> Box<dyn Command> {
//     let cmd = Add::new(args);
//     Box::new(cmd)
// }
