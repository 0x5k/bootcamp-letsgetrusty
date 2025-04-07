fn main() {
    //tuple
    let rgb_color = (0, 255, 111);
    let cmyk_color = (100, 11, 110, 10);

    // tuple struct
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let color1 = RGB(0, 255, 111);
    let color2 = CMYK(100, 11, 110, 10);

    // unit like struct, empty struct used as a marker for traits
    struct RareUnitStruct;
}
