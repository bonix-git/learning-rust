// #![allow(unused)] // For beginning only.

// Definir o modulo prelude
pub mod prelude;

use prelude::*
;
fn main(){

    //secret_number();
    casting_types();

}


fn casting_types(){

    let g:f32 ;
    g = -4.99;
    let h = g as i32;

    let c = "1234";

    let c = 1234;

    println!{"\n g = {}", g};

    println!{"\n h = {}", h};

    println!{"\n g = {}", g};

    println!{"\n c = {}", c};
}


fn secret_number(){
    println!("\n\tProgramar é legal!\n\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("\tO número secreto é: {secret_number}\n");

    loop {
        // Prompt de entrada com tabulação
        print!("\tInsira um número: ");
        // Assegura que o prompt seja impresso antes de ler a entrada
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler");

        let guess: u32 = guess.trim().parse().expect("\tPor favor, digite um número: ");

        println!("\n\tSeu chute é: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\tMuito pequeno!\n"),
            Ordering::Greater => println!("\tMuito grande!\n"),
            Ordering::Equal => {
                println!("\tVocê venceu!");
                break;
            }
        }
    }
}