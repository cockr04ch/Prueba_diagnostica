use std::{usize};



//Funcion Que devolvera si la cadena es Fen o no
fn validar_fen(s:&str) -> bool{

    if !validar_partes(s) {
        return false; 
    }

    let part_tablero: Vec<&str> = s.split_whitespace().collect();
    let tablero = part_tablero[0];

    if !valdidar_segmento(tablero) {
       return false; 
    }

    //turno
    if part_tablero[1] != "w" && part_tablero[1] != "b"{
        return false;
    }

    true
}

//Funcion Para validar que la cadena tenga exactamente 6 partes
//Parte 1 -> "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
//Parte 2 -> "w" Turnos
//Parte 3 -> "KQkq" Enroques
//Parte 4 -> "-" captura del paso
//Parte 5 -> "0" contador de movimientos
//Parte 6 -> "1" Movimiento
fn validar_partes(s:&str) -> bool {
    let partes: Vec<&str>  = s.split_whitespace().collect();
    println!("{}",partes[0]);
    if partes.len() != 6 {
        return false
    }else {
        return true
    }
}

fn valdidar_segmento(s:&str) -> bool {

    let filas:Vec<&str> = s.split('/').collect();

    if filas.len() !=8 {
        return false; 
    }

    for cada_fila in filas {
        let mut carac_valido = 0;
        for caracter in cada_fila.chars() {
            if "rnbqkbnrpRNBQKBNRP".contains(caracter) {
                carac_valido +=1;
            }else if  caracter.is_digit(10){
                carac_valido += caracter.to_digit(10).unwrap() as usize;
            }else {
                return false;
            }
        }

        if carac_valido !=8 {
            return false;
        }
    }

    true
}



fn main() {
    println!("Hello, world!");
    let fen = "rnbqkbnr/ppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    //validar_fen(fen);
    match validar_fen(fen) {
        true => {
            println!("Es Fen");
        }

        false =>{
            println!("No Fen");
        }
        
    }
}
