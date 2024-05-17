use std::io;

fn operation(x:f64,y:f64,opr:char)->f64 {
    let mut result: f64 = 0.0;
    match opr {
        '+' => result = x + y,
        '-' => result = x - y,
        '*' => result = x * y,
        '/' => result = x / y,
        _ => println!("Input inválido!"),
    };
    result
}

fn calculator(){
    let mut var1 = String::new();
    let mut var2 = String::new();

    println!("Digite o primeiro valor: ");
    io::stdin().read_line(&mut var1).expect("Erro");

    println!("Digite o segundo valor: ");
    io::stdin().read_line( &mut var2).expect("Erro");

    let mut opr = String::new();
    println!("Digite a operação: ");
    io::stdin().read_line( &mut opr).expect("Erro");
    let op: Vec<char> = opr.chars().collect();

    let var1: f64 = var1.trim().parse().expect("Digite um número!");
    let var2: f64 = var2.trim().parse().expect("Digite um número!");

    println!("O resultado é : {}", operation(var1,var2,op[0]));
}

fn main() {
    println!("Calculadora no RUST!");
    let mut looper = String::new();
    loop{
        calculator();
        
        println!("Deseja continuar ?");
        io::stdin().read_line( &mut looper).expect("Erro");
        
        if looper.trim() == "Não" {break;}
    }
}
