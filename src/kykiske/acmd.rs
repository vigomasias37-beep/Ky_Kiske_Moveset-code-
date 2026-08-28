//use super::*;

pub mod jabs;
pub mod dash_attack;
pub mod tilts;
pub mod smashes;
pub mod aerials;
pub mod specials;
pub mod throws;
pub mod misc;


pub fn install() {

    jabs::install();
    dash_attack::install();
    tilts::install();
    smashes::install();
    aerials::install();
    specials::install();
    throws::install();
    misc::install();

}