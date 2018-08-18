extern crate space_rx;
extern crate failure;

use space_rx::SortDir;
use space_rx::v2_api::requests::info::*;

fn main() -> Result<(), String> {
    let cr = InfoHistoryRequestBuilder::default().flight_number(14).build()?;
    let res = space_rx::send(cr);
    //println!("{:?}", cr);
    println!("{:?}", res);
    Ok(())
}