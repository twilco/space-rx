extern crate space_rx;
extern crate failure;

use space_rx::SortDir;
use space_rx::v2_api::requests::info::*;
use space_rx::v2_api::requests::launch::*;
use space_rx::v2_api::requests::part::*;

fn main() -> Result<(), String> {
    let cr = CorePartRequestBuilder::default().core_serial("B1041").build()?;
    let res = space_rx::send(&cr);
    //println!("{:?}", cr);
    println!("{:?}", res);
    Ok(())
}