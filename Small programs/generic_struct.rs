//How to access generic struct fields after passing it
//as a generic parameter

trait Flight<T>{
    fn speed(&self) -> T;
}

struct Bird<T>{
    pigeon: T,
}

//Copy trait is needed so you don't have &&
impl <T>Flight<T> for Bird<T>
    where T: Copy{
    fn speed(&self) -> T{
        self.pigeon
    }
}

//U has the created Flight trait to access
//fields in struct Bird
fn send<U,T>(val: &U) -> T
    where U: Flight<T>{
    val.speed()
}
fn main() {
    let bird = Bird{
        pigeon: 65.4,
    };
    
    let bird2 = Bird{
        pigeon: "fly away",
    };
    let s = send(&bird);
    let b = send(&bird2);
    
    println!("{}",s);
    println!("{}",b);
}
