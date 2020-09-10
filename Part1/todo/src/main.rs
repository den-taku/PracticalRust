enum Emotion {
    Anger,
    Happy
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

struct HappyPerson {
    name: String,
    state: Emotion
}

impl Emotional for HappyPerson {
    fn get_anger(&mut self) -> String {
        unimplemented!()
    }

    fn get_happy(&mut self) -> String {
        format!("{} is always happy.", self.name)
    }

    fn tell_state(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy
    };
    println!("{}", p.get_happy());
}
