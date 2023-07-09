use crate::types;


pub enum TrafficLight {
    RED(RED),
    YELLOW(YELLOW),
    GREEN(GREEN),
}

pub struct RED {}
pub struct YELLOW {}
pub struct GREEN {}

pub struct People {
    pub action: String,
}

impl RED {
    fn transition(&self, people: &mut People) {
        people.action = String::from("stop");
    }
}

impl YELLOW {
    fn transition(&self, people: &mut People) {
        people.action = String::from("slow down");
    }
}

impl GREEN {
    fn transition(&self, mut people: &mut People) {
        people.action = String::from("go");
    }
}

pub struct StateMachine<S> {
    pub current_state: S,
    pub previous_state: S,
}

// #[derive(Default)]
// pub struct Storage {}

// pub struct S3 {}


impl <S> types::NextState for StateMachine<S> {
    type State = S;

    fn next(&self, state: Self::State) {
        todo!()
    }
}

// impl types::NextStateContext for StateMachine {
//     type State = u8;
//     type Context = Storage;


//     fn next(&self, state: Self::State, context: Self::Context) {
//         todo!()
//     }

// }

#[cfg(test)]
mod tests {
    use super::*;

    fn change_light(traffic_light: TrafficLight, people: &mut People) {
        match traffic_light {
            TrafficLight::GREEN(item) => {
                item.transition(people);
            },
            TrafficLight::YELLOW(item) => {
                item.transition(people);
            },
            TrafficLight::RED(item) => {
                item.transition(people);
            },
        };
    }


    #[test]
    fn simple_case() {
        let yellow = TrafficLight::YELLOW(YELLOW {});
        let red = TrafficLight::RED(RED {});
        let green = TrafficLight::GREEN(GREEN {});
        let mut people = People { action: String::from("stop") };

        change_light(yellow, &mut people);

        println!("People action: {:?}", &people.action);

        change_light(red, &mut people);

        println!("People action: {:?}", &people.action);

        change_light(green, &mut people);

        println!("People action: {:?}", &people.action);
    }
}
