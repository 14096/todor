pub mod modules;

use anyhow::Result;
pub use modules::app::App;

pub fn run() -> Result<()> {
    let mut app = App::new()?;
    app.run()
}
