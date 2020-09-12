fn type_of<R>(_:&R) -> String {
    std::any::type_name::<R>().to_string()
}

macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! vec {
    ( $x:ty ) => {
        {
            let temp_vec: Vec<$x> = Vec::new();
            temp_vec
        }
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    assert_eq!(five_times!(2 + 3), 25);
    let x = vec![0];
    println!("{}: {:?}", type_of(&x), x);
    let y = vec![0, 1, 2];
    println!("{}: {:?}", type_of(&y), y);
    let z = vec![i32];
    println!("{}: {:?}", type_of(&z), z);
}
