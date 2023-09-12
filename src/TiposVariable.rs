
fn main() {

    let _mi_variable = 0; // Las variables que nunca se usan empiezan con _
    const MI_CONSTANT: u8 = 0; //Segun yo, significa que puedes usar 8 bits para ese numero entero
    static MI_STATIC: u8 = 0;

    let mi_numero7 = 100;
    println!("{}", mi_numero7 as u8 as char);

// Incluso es mejor:
    let mi_numero: u8 = 100; // se indica de forma expresa que
                             //el tipo de la variable mi_numero es u8
    println!("{}", mi_numero as char);

//     //////////////////////////////////////////////////////////////////////////////////

// 1 Byte: Letras, números y símbolos de operaciones aritméticas, $, @
// 2 Byte: Letras con acentos y diéresis, algunos signos, á, ä, ë, !,…
// 3 Byte: Caracteres coreanos, japoneses, chinos, 国 안 녕 …
// 4 Byte: Emojis y otros caracteres menos usados

    println!("Tamaño de un char: {}", std::mem::size_of::<char>()); // Resultado 4 bytes
// .len() devuelve el tamaño de una cadena de texto en bytes
    println!("Tamaño de una cadena que contiene la 'a': {}", "a".len());
    println!("Tamaño de una cadena que contiene la 'ß': {}", "ß".len());
    println!("Tamaño de una cadena que contiene la '国': {}", "国".len());
    println!("Tamaño de una cadena que contiene la '𓅱': {}", "𓅱".len());

    let fragmento = "¡Hola!";
    println!("El fragmento ocupa {} bytes.", fragmento.len());
    let fragmento2 = "안녕!"; // Coreano de "hola"
    println!("El fragmento2 ocupa {} bytes.", fragmento2.len());

    //     La "_" se usa para mayor legibilidad y no importa cuantas uses
//     let numerito = 10u8;
    let numerito = 10_u8;
    let numerazo = 100_000_000_i32;

    let numero = 0________u8;
    let numero2 = 1___6______2____4______i32;
    println!("{}, {}", numero, numero2);

    //     Para indicar que un número va a ser decimal (float), se puede hacer de las siguientes maneras:
    let mi_decimal = 5.; // Rust ve un . y sabe que es un decimal (float, en inglés)
    let mi_decimal2: f64 = 5.0; // Esta variable es de tipo f64
    let mi_otro_decimal: f32 = 8.5; // Esta es de tipo f32

    //     Para sumar estos dos números hay que convertir el f32 en f64 primero, así que:

    let tercer_decimal = mi_decimal + mi_otro_decimal as f64;

    // Variables mutables

    let mut mi_numero5 = 8; //No se puede cambiar el tipo de variable, pero si el valor
    mi_numero5 = 10;

    // Ocultar variable
    // Se asignan distintos valores a variables aparentemente iguales pero que son distintas porque contienen datos distintos, con el mismo nombre
    // Si estan en distintos bloques se puede acceder a una variable anterior pero si no lo estan, se superpone y solo se puede acceder a la ultima
    let mi_numero6: u32 = 8;
    let mi_numero6 = "Miau";
    let mi_numero6: f32 = 8.;
    // println!("Mi_numero6 tiene dentro {} de tamaño {}", mi_numero6, mi_numero6.len());

    fn mis_cadenas() {
        let _mi_str: &str = "Hola mundo"; // El valor no puede cambiar, tiene tamaño fijo
        let _mi_cadena: String = String::from("Hola, mundo"); // Tamaño desconocido cuando se compila
        let _mi_cadena2 = String::from("El zorro marron rapido");
        // let _mi_str2: &str = &my_string[3..8]; // "zorro"
    }
    fn mi_array() {
        let _primer_array: [usize; 5] = [1, 2, 3, 4, 5];
        // let _mi_array_slice: &[usize] = &my_array[0..3]; // [1, 2, 3]

    }
    fn mi_func(x: u8) -> i32 {
        x as i32 // convertir variable con parametro x de 8 bits a entero positivo-negativo de 32
    }
}