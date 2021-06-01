enum Enum {
    Enum1,
    Enum2,
}
struct Structura {
    value_i32: i32,
    value_boolean: bool,
}
struct TupleStructura(f64, f32, i8, u32, char);
fn function_enum1() -> Enum {
    Enum::Enum1
}
fn function_enum2() -> Enum {
    Enum::Enum2
}
fn f2() -> Structura {
    Structura {
        value_i32: 32,
        value_boolean: true,
    }
}
fn f3() -> TupleStructura {
    TupleStructura(4.7, 2., 3, 5, 'w')
}
fn f4() -> [u8; 4] {
    [7, 2, 0, 19]
}
fn f5() -> Vec<u8> {
    vec![120]
}

fn main() {
    println!(
        "{} {}",
        match function_enum1() {
            Enum::Enum1 => 10,
            Enum::Enum2 => 20,
            
        },
        match function_enum2() {
            Enum::Enum1 => true,
            Enum::Enum2 => false,
            
        }
    );
    println!("{} ", f2().value_i32);
    println!("{} ", f2().value_boolean);
    println!("{} ", f3().0);
    println!("{} ", f4()[0]);
    println!("{} ", f5()[0]);
}
