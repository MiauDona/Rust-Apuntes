fn main() {
    //     Los “comandos” que acaban con ! son macros. “Escriben” código por ti.
//     println: - Las llaves son para meter variables dentro de ellas.
    println!("¡Hola, mundo número {}!", 8);
    println!("¡Hola, mundos número {} y {}!", 8, 9);

    // print! hace que lo siguiente no salte de linea
    print!("No salto de linea porque");
    println!(" no quiero");


    // pista: std::i8::MIN significa
    // "el valor de MIN de la sección i8 de la librería estandar"
    println!("El menor i8 es {} y el mayor i8 es {}.", std::i8::MIN, std::i8::MAX);
    println!("El menor u8 es {} y el mayor u8 es {}.", std::u8::MIN, std::u8::MAX);
    println!("El menor i16 es {} y el mayor i16 es {}.", std::i16::MIN, std::i16::MAX);
    println!("El menor u16 es {} y el mayor u16 es {}.", std::u16::MIN, std::u16::MAX);
    println!("El menor i32 es {} y el mayor i32 es {}.", std::i32::MIN, std::i32::MAX);
    println!("El menor u32 es {} y el mayor u32 es {}.", std::u32::MIN, std::u32::MAX);
    println!("El menor i64 es {} y el mayor i64 es {}.", std::i64::MIN, std::i64::MAX);
    println!("El menor u64 es {} y el mayor u64 es {}.", std::u64::MIN, std::u64::MAX);
    println!("El menor i128 es {} y el mayor i128 es {}.", std::i128::MIN, std::i128::MAX);
    println!("El menor u128 es {} y el mayor u128 es {}.", std::u128::MIN, std::u128::MAX);
}