def main():
    n = 100  # Fijamos n=100 como solicitaste
    x = leer_valor_x()
    
    coeficientes = generar_coeficientes(n)
    imprimir_polinomio_abreviado(coeficientes)
    evaluar_polinomio(coeficientes, x)

def leer_valor_x():
    """Solicita al usuario el valor de x"""
    while True:
        try:
            x = float(input("Ingrese el valor de x (número real): "))
            return x
        except ValueError:
            print("Entrada inválida. Por favor, ingrese un número válido.")

def generar_coeficientes(n):
    """
    Genera los coeficientes binomiales para (1 + x)^n
    usando el triángulo de Pascal
    """
    if n == 0:
        return [1]
    
    # Iniciamos con la fila [1, 1] que corresponde a n=1
    fila_anterior = [1, 1]
    
    # Generamos fila por fila hasta llegar a n
    for i in range(2, n + 1):
        fila_actual = [1]  # Siempre empieza con 1
        
        # Calculamos los elementos internos
        for j in range(1, i):
            fila_actual.append(fila_anterior[j - 1] + fila_anterior[j])
        
        fila_actual.append(1)  # Siempre termina con 1
        fila_anterior = fila_actual
    
    return fila_anterior

def imprimir_polinomio_abreviado(coeficientes):
    """
    Muestra una versión abreviada del polinomio ya que para n=100
    sería demasiado largo mostrar todos los términos
    """
    print(f"\nPolinomio: (1 + x)¹⁰⁰")
    print(f"Número de términos: {len(coeficientes)}")
    print(f"Coeficientes (primeros 5): {coeficientes[:5]}")
    print(f"Coeficientes (últimos 5): {coeficientes[-5:]}")
    print(f"Coeficiente máximo: {max(coeficientes)}")
    print(f"Suma de coeficientes: {sum(coeficientes)}")

def evaluar_polinomio(coeficientes, x):
    """
    Evalúa el polinomio para el valor de x dado
    """
    resultado = 0.0
    print(f"\nEvaluando (1 + {x})¹⁰⁰:")
    
    # Usamos la fórmula directa para verificación
    resultado_directo = (1 + x) ** 100
    
    # Evaluación término por término
    for i, coef in enumerate(coeficientes):
        termino = coef * (x ** i)
        resultado += termino
        
        # Mostramos algunos términos representativos
        if i in [0, 1, 2, 50, 98, 99, 100]:
            print(f"  Término {i}: C(100,{i}) × {x}^{i} = {coef} × {x**i} = {termino}")
    
    print(f"\nResultado por evaluación término a término: {resultado}")
    print(f"Resultado por fórmula directa (1+x)¹⁰⁰: {resultado_directo}")
    print(f"Diferencia: {abs(resultado - resultado_directo)}")

if __name__ == "__main__":
    main()
