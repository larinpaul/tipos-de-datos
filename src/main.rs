fn main() {
    
    // Tipos escalares
    // Un tipo escalar representa un valor único.
    // Rust tiene cuatro tipos escalares principales:
    // enteros, números en coma flotante (decimals), booleanos y caracters.

    // Tipos enteros
    // Un entero es un númnero sin componente decinal.
    // Longitud // Con signo    // Sin signo
    // 8-bit    // i8           // u8
    // 16-bit   // i16          // u16
    // 32-bit   // i32          // u32
    // 64-bit   // i64          // u64
    // 128-bit  // i128         // u128
    //          // isize        // usize
    
    // Rango de valores
    // Sin signo = 0 ... (2^n)-1
    // Con signo = (2n^1) ... (2n^-1)-1

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 366999;
    println!("The value of x is: {}", x);

    // Especificar tipo de variable
    // let nombre_variable: tipovariable = valor;

    let mut x:u32 = 5;
    println!("The value of x is: {}", x);
    x = 366999;
    println!("The value of x is: {}", x);

    // let mut x:u32 = 5;
    // println!("The value of x is: {}", x);
    // x = -366999; // error!
    // println!("The value of x is: {}", x);

    // Tipos en coma flotante (decimales)
    // Rust también tiene dos tipos primitivos para números de punto flotante,
    // que son números con puntos decimales. Los tipos de punto flotante de
    // Rust son f32 y f64, que tienen un tamaño de 32 bits y 64 bits,
    // respectivamente.

    let decimal1 = 4.0;
    println!("{}",decimal1);
    let decimal2: f32 = -455.399;
    println!("{}",decimal2);

    // Operaciones básicas con números
    // Rust admite las operaciones matemátical básicas que se encuentran en la
    // mayoría de los lenguajes: suma, resta, multiplicación, división y resto.

    let suma = 58 + 67;
    println!("{}",suma);
    let resta = 34343.39 - 788.3;
    println!("{}",resta);
    let multiplicacion = 4455.3 * 23.0;
    println!("{}",multiplicacion);
    let division = 34 /4;
    println!("El resultado de 34/4 debe ser 8: {}", division);
    let resto = 34 % 4;
    println!("{}", resto);

    // Tipo booleano
    // Como en la mayoría de los lenguajes de programación, un tipo booleano en Rust
    // tiene dos valores posibles: true y false. Los booleanos tienen un tamaño de un byte.
    // El tipo booleano en Rust se especifica mediante la palabra reservada bool.

    let verdadero = true;
    println!("{}", verdadero);
    let falso: bool = false;
    println!("{}", falso);

    // Tipo carácter
    // Rust admite el tipo char, que se almacena en comilas simples, al contrario que las
    // cadenas que se almacenarán con comillas dobles.
    // El tipo char en Rust tienen un tamaño de cuarto bytes y representa un valor en
    // Unicode, lo que significa que podemos representar mucho más que con ASCII:
    // letras acentuadas, carácteres de otros idiomas o emojis.

    let a = 'a'; // usa 4 bytes
    println!("{}", a);
    let emoji = '😊';
    println!("{}", emoji);

    // Tipos compuestos
    // Los tipos compuestos pueden argupar varios valores en un solo tipo.
    // Rust tiene dos tipos de compuestos primitivos:
    // tuplas y arrays (matrices).

    // Tipo tupla
    // Una tupla es una forma general de agrupas varios valores, que pueden ser de
    // diferenctes tipos, en un tipo compuesto.
    // Las tuplas tienen una longitud fija: una vez declaradas,
    // no pueden crecer ni encogerse de tamaño.

    // Las tuplas se crean escribiendo una lista de valores separados por comas entre
    // paréntesis. Cada posición en la tupla tiene un tipo, y los tipos de los diferentes
    // valores en la tupla no tienen que ser iguales.

    let tupla:(f32,u8,f32) = (500.34, 87, -344.9);

    let (x,y,z) = tupla;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

    let primero = tupla.0;
    println!("El primero elemento de la tupla: {}", primero);

    // Tipo array (matriz)
    
    // Los arrays también nos van permitir almacenar una colección de valores múltiples.
    // A diferencia de las tupla, todos los elementos de una matriz, deben tener el mismo
    // tipo. Las matrices en Rust son de longitud fija, como las tuplas.

    // En Rust, los valores que componen un array se escriben como una lista separada
    // por comas entre corchetes.

    let matriz = [23,45,2,11,-45];

    let x = matriz[0];

    println!("{}",x);

    let matriz = [3; 5];
    // let matriz = [3, 3, 3, 3, 3];
    println!("Array: {:?}", matriz);
    // In this example, the println! macro is used to print the array matriz
    // to the standard output. The {:?} placeholder in the format string
    // indicates that the argument should be formatted using the debug
    // trait, which is a built-in trait in Rust that provides a default
    // formatting for printing the value of a variable.  

}
