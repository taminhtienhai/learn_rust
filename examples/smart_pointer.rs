
trait VehicleControl {}

struct Car(u32);
struct Bike(u32);

impl VehicleControl for Car {}
impl VehicleControl for Bike {}


struct VehicleStore<T: VehicleControl> {
    pub vehicles: Vec<T>,
}

// impl <T> VehicleStore<T> {
//     pub fn new() -> Self {
//         Self { vehicles: vec![], _type: PhantomData }
//     }
// }

impl <T: VehicleControl> VehicleStore<T> {
    pub fn add_vehicle(&mut self, vehicle: T) {
        self.vehicles.push(vehicle);
    }
}

fn main() {
    let store = VehicleStore { vehicles: Vec::<Car>::new() };
}