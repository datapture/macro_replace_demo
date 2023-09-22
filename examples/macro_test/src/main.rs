
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



use mreplace::macro_replace;

fn main() {
    // let v0:i32 = 5;
    // let value = macro_replace!({
    //     let v1:Option<@keywork> = None;        
    //     if v0 > @keywork::c1() {
    //         v1 = Some(@keywork::new(v0));
    //     }
    //     else {
    //         v1 = Some(@keywork::new(@keywork::c2()));
    //     }
    //     v1
    // }, Mytype);

    let param1:f64 = 0.5;
    let param2:f64 = 10.7;
    let value:f64 = macro_replace!({param2 + @keyword(param1) },{ f64::asin }); 

    println!("Hello, {}", value);
}
