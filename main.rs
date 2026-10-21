 use std::io;
fn rango() {
    let mut entrada = String::new();

    println!("Ingrese el valor inicial:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer el dato");

    let inicio: i32 = entrada.trim().parse()
        .expect("Debe ingresar un numero entero");

    entrada.clear();

    println!("Ingrese el valor final:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer el dato");

    let fin: i32 = entrada.trim().parse()
        .expect("Debe ingresar un numero entero");

    let mut impares: Vec<i32> = Vec::new();

    if inicio <= fin {
        for numero in inicio..=fin {
            if numero % 2 != 0 {
                impares.push(numero);
            }
        }
    }

    println!("\nNumeros impares encontrados: {:?}", impares);

    let cantidad = impares.len();

    let suma: i32 = impares.iter().sum();

    println!("Cantidad de numeros impares: {}", cantidad);
    println!("Suma de los numeros impares: {}", suma);

    if cantidad > 0 {
        let promedio = suma as f64 / cantidad as f64;
        println!("Promedio de los numeros impares: {:.2}", promedio);
    } else {
        println!("No se encontraron numeros impares para calcular el promedio.");
    }
} 

fn main(){
 rango();

}

 
