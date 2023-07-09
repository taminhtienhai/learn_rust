mod state_machine;

use state_machine::{TrafficLight, Green};
use proc_enum::*;

#[my_derive]
struct S {}

fn main() {
    let traffic_light = TrafficLight::<Green>::new();

    traffic_light.next().next().next().next();
}
