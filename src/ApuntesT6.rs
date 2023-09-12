

// El ; indica que una función no devuelva lo que hay antes del ; por lo que la función anterior solo ejecuta el println! y no devuelve ningún valor.
// Para que devuelva un valor debe de estar el valor solo o con un return

fn funcion_numero() -> i32 {  //Debe de estar fuera y antes del main
    8
}

// Entran a la función dos i32's. Se llaman num_uno y num_dos
fn multiplicar(num_uno: i32, num_dos: i32) -> i32{
    let resultado = num_uno * num_dos;
    println!("{} por {} es {}", num_uno, num_dos, resultado);
    resultado //No hay por que poner un return, no tiene ";".
}

fn main() {
    let mi_numero = 100;
    println!("{}", mi_numero as u8 as char);

    // Incluso es mejor:
    let mi_numero: u8 = 100; // se indica de forma expresa que
    // el tipo de la variable mi_numero es u8
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

//     //////////////////////////////////////////////////////////////////////////////////////

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

//     Los “comandos” que acaban con ! son macros. “Escriben” código por ti.
//     println: - Las llaves son para meter variables dentro de ellas.
    println!("¡Hola, mundo número {}!", 8);
    println!("¡Hola, mundos número {} y {}!", 8, 9);

    ///////////////////////////////////////////////////////////////////////////////////////////

    /*El ; indica que una función no devuelva lo que hay antes del ; por lo que la función anterior solo ejecuta el println!
    y no devuelve ningún valor. Para que devuelva un valor debe de estar el valor solo o con un return */
    println!("¡Hola, mundo número {}!", funcion_numero());

    multiplicar(8, 9); // Pasamos unos números directamente
    let algun_numero = 10; // o podemos declarar dos variables
    let algun_otro_numero = 2;
    multiplicar(algun_numero, algun_otro_numero); // y pasarlas a la función


    /*Se abre un bloque*/ {
        let mi_numero2 = 8; // mi_numero se crea aquí
                                // mi_numero se extingue aquí
    }
    println!("¡Hola, mundo número {}!", mi_numero2);// mi_numero no existe y
                                                    // println!() no lo puede encontrar

    /*Se puede usar un bloque de código para devolver un valor, como en el siguiente código.*/
    let mi_numero3 = {
        let segundo_num = 8;
        segundo_num + 9 // sin punto y coma, por lo que el
                        // bloque de código devuelve 8 + 9 = 17
    };
    println!("¡Hola, mundo número {}!", mi_numero3);

    // Si se añadiera un punto y coma en la sentencia final del bloque, devolvería () (nada).

    let mi_numero = {
        let segundo_num = 8; // declara el segundo número
        segundo_num + 9;          // suma 9 con el segundo número
                                  // pero no se devuelve
                                  // segundo_num desaparece aquí
    };
    println!("¡Hola, mundo número {:?}!", mi_numero); // mi_numero es ()
                                                      // {:?} sirve para cuando no se puede imprimir algo
                                                      // {:#?} puede ser otra opcion para cuando ocurra

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Variables mutables

    let mut mi_numero5 = 8; //No se puede cambiar el tipo de variable, pero si el valor
    mi_numero5 = 10;

    // Ocultar variable
    // Se asignan distintos valores a variables aparentemente iguales pero que son distintas porque contienen datos distintos, con el mismo nombre
    // Si estan en distintos bloques se puede acceder a una variable anterior pero si no lo estan, se superpone y solo se puede acceder a la ultima
    let mi_numero6 = 8_u32;
    let mi_numero6 = "Miau";
    let mi_numero6 = 8.;
    println!("Mi_numero6 tiene dentro {} de tamaño {}", mi_numero6, mi_numero6.len());

    



}


