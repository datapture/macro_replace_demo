
struct Mytype
{
    dato:String
}

impl Mytype
{
    fn new(numval:i32) -> Self
    {
        Mytype{dato:numval.to_string()}
    }

    fn c1() -> i32 {
        1
    }

    fn c2() -> i32 {
        2
    }
}

//use mreplace::macro_replace;

fn main() {
    // let v0:i32 = 5;
    // let value = macro_replace!({ {        
    //     if v0 > 10 {
    //         @keyword::new(v0)
    //     }
    //     else {
    //         @keyword::new(@keyword::c2())
    //     }
    // } }, {Mytype});

    let param1:f64 = 0.5;
    let param2:f64 = 10.7;
    let value:f64 = mreplace::macro_replace!({param2 + @keyword(param1*param2) },{ f64::asin }); 

    println!("Hello, {}", value);
}
