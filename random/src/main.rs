use rand::random; // Importamos la función random del módulo rand

fn main() {
    let number: u8 = random(); // Generamos un número aleatorio de tipo u8
    println!("Hello, world! tu numero random es: {}", number); // Imprimimos el número aleatorio
}
