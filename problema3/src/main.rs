use std::{env, u8};

fn is_ip(check:&str) -> bool {

    let parts: Vec<&str> = check.split('.').collect();

    if parts.len() !=4 {
        return false;
    }

    parts.iter().all(|&parts| {
        if parts.is_empty() {
            return false;
        }
        parts.parse::<u8>().is_ok()
    })  
}

fn is_email(check:&str) -> bool {

    // Con esto logramos divir el String en dos partes : 
    // ejemplo
    // danielejemplo12@gmail.com
    // parts[0] = danielejemplo12
    // parts[1] = gmail.com
    let parts: Vec<&str> = check.split('@').collect();

    if parts.len() !=2 {
        return false; 
    }

    let ident = parts[0];
    let dominio = parts[1];

    if ident.is_empty() || !ident.chars().all(|char| char.is_alphanumeric()) {
       return false; 
    }

    if dominio.is_empty() || !dominio.contains('.') {
        return false;
    }

    true

}

fn is_notacion(check:&str) -> bool{

    if let Some(posicion) = check.find("x10^") {

        let (mentissa , exponente) = check.split_at(posicion);
        let part_exponente = &exponente[4..] ;


        !mentissa.is_empty() && mentissa.chars().all(|menti| menti.is_numeric() || menti == '.')
            && !part_exponente.is_empty() && part_exponente.chars().all(|c| c.is_numeric())

    }else {
        false
    }
    
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];
    let check = &args[2];

    match flag.as_str() {

        "check" => {
            if is_email(check){
                println!("Es Un Correro Electronico");
            }else if is_ip(check) {
                println!("Es una Direccion IP"); 
            }else if is_notacion(check) {
                println!("Es Una Notacion");
            }else {
                println!("No pertenece a ningun Grupo");
            }
        }

        _ => println!("Fuera de Parametros"),
    }
}
