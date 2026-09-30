use std::io;

fn main() {
    let mut precio = String::new();
    let mut descuento = String::new();

    println!("Ingresa el precio del producto:");
    io::stdin().read_line(&mut precio).unwrap();

    println!("Ingresa el descuento:");
    io::stdin().read_line(&mut descuento).unwrap();

    let precio: f64 = precio.trim().parse().unwrap();
    let descuento: f64 = descuento.trim().parse().unwrap();

    let cantidad = precio * descuento / 100.0;
    let total = precio - cantidad;

    println!("Descuento aplicado: ${}", cantidad);
    println!("Precio total: ${}", total);
}
