//Podstawowe zmienne i typy:
//
// Napisz program, który deklaruje zmienne różnych typów (i32, f64, bool, char) i wyświetla ich wartości.

pub fn run_ex1() {
    let int32 = 1;
    let int8: i8 = 127;
    let uint8: u8 = 255;
    let _int16: i16 = 32_767;
    let int32_2: i32 = 2_147_483_647;
    let uint32_2: u32 = 4294967295;
    let float: f32 = 6.9;
    let boolean = true;
    let char = 'a';

    println!("integer 32bits: {}", int32);
    println!("integer 8bits: {}", int8);
    println!("unsigned integer 8bits: {}", uint8);
    println!("integer int32: {}", int32_2);
    println!("integer uint32_2: {}", uint32_2);
    println!("float: {}", float);
    println!("boolean: {}", boolean);
    println!("char: {}", char);



    let tuple = (8i16, "test", 1.2, true);
    let array = [2u8,3,4,5];
    let vector = vec![42, 2, 4];


    #[derive(Debug)]
    struct TestStruct {
        integer: i8,
        unsigned_integer: u8,
        float: f32,
        boolean: bool,
        char: char,
        tuple: (i8, String, Vec<NestedStruct>),
        vector: Vec<NestedStruct>,
        array: [i8; 5]
    }

    #[derive(Debug)]
    struct NestedStruct {
        a: String,
        b: i32
    }

    let data1 = TestStruct {
        integer: 127,
        unsigned_integer: 255,
        float: 1.1,
        boolean: !array.is_empty(),
        char: 't',
        tuple: (4, String::from("Hello"), vec![NestedStruct {
            a: String::from("nested"),
            b: 44444
        }]),
        vector: vec![NestedStruct {a: String::from("test string"), b: 1}, NestedStruct {a: String::from("test2"), b: 2}],
        array: [1,2,3,4,5]
    };

    println!("log struct {:?}", data1)

}

/*
Ciekawostki z pierwszego zadania:

1. rodzaje cudzysłowia mają znaczenie. Podwójny cudzysłów służy do oznaczania String Literal natomiast pojedynczy służy do oznaczania pojedynczego znaku (char)
2.
 */