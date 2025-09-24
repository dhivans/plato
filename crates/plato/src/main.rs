mod app;
mod textview;

use plato_core::anyhow::Error;
use crate::app::run;

fn main() -> Result<(), Error> {
    run()?;
    Ok(())
}
