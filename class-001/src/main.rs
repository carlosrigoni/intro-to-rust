fn main() {
    // i8, u8, i16, u16, i32, u32, i64, u64, isize, usize,
    // f32, f64

    let _a = 30 - 20;

    let _s = 30 - 20;
    let _m = 5 * 10;
    let _d = 4 / 6;
    let _r = 20 % 20;

    // bool: true/false
    let _c: char = 'z';

    let _t: (i32, f64, char) = (42, 6.12, 'j');
    //  destructuring
    let (_, _y, _x) = _t;
    println!("{}", _t.0);

    // arrays
    let _a = [1, 2, 3, 4, 5, 6, 7, 8];
    let _a1 = _a[0];
}
