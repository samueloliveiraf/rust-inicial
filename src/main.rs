const PI:f32 = 3.14;
static mut VARIAVEL_GLOBAL: u8 = 1;


fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}


fn escopo_1() {
    let a = 123;

    {
        let b = 456;
        println!("DENTRO b = {}", b);
    }

    println!("DENTRO a = {}", a);

}


fn escopo() {
    unsafe {
        println!("Global {}, tamanho {} bytes", VARIAVEL_GLOBAL, std::mem::size_of_val(&VARIAVEL_GLOBAL));
    }

    println!("PI = {}, tamanho = {} bytes", PI, std::mem::size_of_val(&PI));

    let variavel = 1;
    println!("Variável = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("Váriavel 2 = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));

    let boo:bool = false;
    println!("Váriavel booleano = {}, tamanho {} bytes", boo, std::mem::size_of_val(&boo));

    let letra:char = 'C';
    println!("Váriavel char = {}, tamanho {} bytes", letra, std::mem::size_of_val(&letra));
}


fn condicoes(idade: u8, resposavel: bool) {
    if idade >= 18 || idade >= 16 && resposavel {
        println!("Pode entrar na balada!");
    } else {
        println!("Não pode entrar na balada!");
    }

    let condicao = if idade >= 18 {"maior"} else {"menor"};

    println!("é {} de idade", condicao);
}

fn repeticoes(num: u8, lin: &str) {
    let multiplicador:u8 = num;

    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;

    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

    let linguagem = lin;
    let proposito = match linguagem {
        "php" => "web",
        "kotli" => "andriod",
        "python" => "data science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} é {}", linguagem, proposito);

}


fn main() {
    escopo();
    escopo_1();
    soma(5, 10);
    condicoes(16, true);
    repeticoes(8, "java");
}
