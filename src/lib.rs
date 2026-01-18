pub mod app;
pub mod storage;
pub mod todo;
pub mod ui;

use anyhow::Result;
pub use app::App;

pub fn run() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
