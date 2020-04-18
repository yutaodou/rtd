mod add;
mod cmd;
mod done;
mod list;
mod today;

pub use add::Add;
pub use done::Done;
pub use list::List;
pub use today::Today;

pub trait Command {
    fn run(self: &Self) -> Result<(), &'static str>;
}
