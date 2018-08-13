extern crate space_rx;

//use space_rx::send_request;
use space_rx::v2_api::*;

fn main() {
    let cr = CompanyRequestBuilder::default().build().unwrap().send();

    println!("{:?}", cr);
}