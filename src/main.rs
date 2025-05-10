use vec::MyVec;

mod vec;

fn main() {
    let mut vetor = MyVec::<i32>::new();

    println!("Meu vetor: {:?}", vetor);
    println!("Tamanho do meu vetor: {}", vetor.len());

    for index in 0..3 {
        vetor.push(index);

        println!("\nElemento do meu vetor: {:?}", vetor[index as usize]);
        println!("Tamanho do meu vetor: {}", vetor.len());
    }

    vetor.clear();

    println!("\nVetor: {:?}", vetor);

    let mut vetor = MyVec::<String>::with_capacity(2);

    vetor.push(String::from("Teste1"));
    vetor.push(String::from("Teste2"));

    println!("\nVetor: {:?}", vetor);
    println!("Primeiro: {}", vetor[0]);
    vetor.pop();
    println!("\nVetor: {:?}", vetor);
    println!("Primeiro: {}", vetor[0]);
}
