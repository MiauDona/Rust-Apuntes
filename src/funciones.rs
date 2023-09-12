

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
    println!("{}, {}", funcion_numero(), multiplicar(-4, 10));

    /*Se abre un bloque*/ {
        let _mi_numero2 = 8; // mi_numero se crea aquí
        // mi_numero se extingue aquí
    }
    // println!("¡Hola, mundo número {}!", mi_numero2);// mi_numero no existe y
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
}