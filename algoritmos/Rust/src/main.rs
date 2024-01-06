mod pesquisa_binaria;

use rand::{Rng, thread_rng};

use pesquisa_binaria::binary_search;

fn main() {
    let mut rng = thread_rng();
    let mut array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    array.sort();
    println!("The array is {:?}", array);
    let item = rng.gen_range(0..11);

    println!("Minha lista: {:?}\n Número que estou buscando: {}\n O valor se encontra na posição: {}", array, item,
             binary_search(&array, &item).unwrap())
}
