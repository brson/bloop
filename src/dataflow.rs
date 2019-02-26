use crate::{Result, Error};

use failure::err_msg;
use timely;
use timely::dataflow::operators::*;
use timely::communication::initialize::WorkerGuards;

pub fn do_your_thing() -> Result<()> {
    debug!("dataflow experiment");

    let r = timely::execute_from_args(std::env::args(), |worker| {

    });

    r.map_err(|e| err_msg(e))?;

    Ok(())
}
