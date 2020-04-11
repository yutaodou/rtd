mod add;
mod cmd;
mod done;
mod list;

pub use add::Add;
pub use done::Done;
pub use list::List;

pub trait Command {
    fn run(self: &Self) -> Result<(), &'static str>;
}
