//Inicia la calculadora
use regex::Regex;

fn main() {
    println!("Bienvenida a la calculadora.");

    //regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    //Traer datos del usuario
    println!("Por favor, introduce tu expresión: ");

    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //suma
    loop {
        //aplicar las operaciones
        let caps = re_add.captures(expression.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    //mostrar el resultado
    println!("Resultado de la suma es: {}", expression);
}
