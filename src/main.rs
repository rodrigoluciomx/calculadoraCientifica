use regex::Regex;

fn main() {
    println!("Hello, world!");
    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // Traer datos de usuario
    println!("Ingrese la operación a realizar: ");
    let mut operacion = String::new();
    std::io::stdin().read_line(&mut operacion).unwrap();


    // Aplicar Operaciones
    let caps = re_add.captures(operacion.as_str()).unwrap();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value + right_value;
    println!("{:?} izq: {}, der: {}",caps,left_value,right_value);

    // Mostrar Resultados
    println!("El resultado es: {}", addition);

}
