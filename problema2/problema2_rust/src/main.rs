use std::io;

fn main() {
    let n = leer_entero_no_negativo();
    let x = leer_valor_x();

    let coeficientes = generar_coeficientes(n);
    imprimir_polinomio(&coeficientes);
    evaluar_polinomio(&coeficientes, x);
}

fn leer_entero_no_negativo() -> usize {
    loop {
        println!("Ingrese el valor de n (entero no negativo):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) if num >= 0 => return num,
            _ => println!("Entrada inválida. Por favor, ingrese un entero no negativo."),
        }
    }
}

fn leer_valor_x() -> f64 {
    println!("Ingrese el valor de x (número real):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("x debe ser un número válido")
}

fn generar_coeficientes(n: usize) -> Vec<u128> {
    let mut triangulo = Vec::with_capacity(n + 1);
    triangulo.push(vec![1]); // Fila 0

    for i in 1..=n {
        let mut fila = Vec::with_capacity(i + 1);
        fila.push(1); // Primer elemento siempre es 1

        for j in 1..i {
            let valor = triangulo[i - 1][j - 1] + triangulo[i - 1][j];
            fila.push(valor);
        }

        fila.push(1); // Último elemento siempre es 1
        triangulo.push(fila);
    }

    triangulo[n].clone()
}

fn imprimir_polinomio(coeficientes: &[u128]) {
    let terminos: Vec<String> = coeficientes
        .iter()
        .enumerate()
        .map(|(i, &coef)| {
            if i == 0 {
                coef.to_string()
            } else {
                format!("{}x^{}", coef, i)
            }
        })
        .collect();
    println!("Coeficientes: {:?}", coeficientes);
    println!("Polinomio: {}", terminos.join(" + "));
}

fn evaluar_polinomio(coeficientes: &[u128], x: f64) {
    let mut resultado = 0.0;
    println!("Evaluación del polinomio para x = {}:", x);
    for (i, &coef) in coeficientes.iter().enumerate() {
        let termino = coef as f64 * x.powi(i as i32);
        println!("  Paso {}: {} * {}^{} = {}", i, coef, x, i, termino);
        resultado += termino;
    }
    println!("Resultado final: {}", resultado);
}
