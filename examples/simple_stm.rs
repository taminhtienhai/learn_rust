use std::marker::PhantomData;

pub trait LightControl {}

pub struct Red;
pub struct Green;
pub struct Yellow;

impl LightControl for Red { }
impl LightControl for Green {}
impl LightControl for Yellow {}

pub struct TrafficLight<Color: LightControl> {
    _color: PhantomData<Color>,
}

impl <Color: LightControl> TrafficLight<Color> {
    pub fn new() -> Self {
        Self { _color: PhantomData::<Color> }
    } 
}

impl TrafficLight<Red> {

    pub fn next(self) -> TrafficLight<Green> {
        println!("Light turn Green");
        return TrafficLight::<Green>::new()
    }
}

impl TrafficLight<Green> {
    pub fn next(self) -> TrafficLight<Yellow> {
        println!("Light turn Yellow");
        return TrafficLight::<Yellow>::new()
    }
}

impl TrafficLight<Yellow> {
    pub fn next(self) -> TrafficLight<Red> {
        println!("Light turn Red");
        return TrafficLight::<Red>::new()
    }
}

fn main() {
    let traffic_light = TrafficLight::<Green>::new();

    let state_01 = traffic_light.next();
    let state_02 = state_01.next();
    let state_03 = state_02.next();
    let state_04 = state_03.next();
}