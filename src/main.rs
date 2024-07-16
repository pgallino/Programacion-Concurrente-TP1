mod processors;
mod structs;
mod test;

use processors::{list_files, process_files, process_totals};
use rayon::ThreadPoolBuilder;
use serde_json::to_string_pretty;
use std::env;
use std::time::Instant;

/// Setea el número de workers
fn configure_workers() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Seteo de workers erroneo, debe ingresar como argumento la cantidad deseada\n");
        std::process::exit(1);
    }

    let num_threads: usize = args[1].parse().expect("Se requiere un número entero");
    match ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
    {
        Ok(_) => {}
        Err(e) => eprintln!("Error al crear ThreadPool: {}", e),
    }
}

fn main() {
    // se setea la cantidad de workers
    configure_workers();

    let start = Instant::now();

    let files = list_files("/data");

    // se obtiene una estructura con la forma del json final
    let mut result_data = process_files(&files);

    // se calculan los totals sobre lo procesado
    process_totals(&mut result_data);

    // Imprime la cadena JSON resultante
    let json_string =
        to_string_pretty(&result_data).expect("Error al serializar el HashMap a JSON");
    println!("{}", json_string);
    eprintln!("Tiempo transcurrido: {:?}", start.elapsed());
}
