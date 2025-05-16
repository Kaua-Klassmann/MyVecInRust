use vec::MyVec;

mod vec;

fn main() {
    let mut vetor = MyVec::<i32>::new();

    println!("Tamanho do meu vetor: {}", vetor.len());

    for index in 0..3 {
        vetor.push(index);

        println!("\nElemento do meu vetor: {:?}", vetor[index as usize]);
        println!("Tamanho do meu vetor: {}\n", vetor.len());
    }

    vetor.clear();

    let mut vetor = MyVec::<String>::with_capacity(2);

    vetor.push(String::from("Teste1"));
    vetor.push(String::from("Teste2"));

    println!("Primeiro: {}\n", vetor[0]);
    vetor.pop();
    vetor.insert(0, String::from("Teste0"));
    println!("Primeiro: {}", vetor[0]);
    println!("Segundo: {}", vetor.get(1).unwrap());
}
