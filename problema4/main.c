#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Función para leer y mostrar el contenido del archivo
void leer() {
    FILE *archivo = fopen("reservadas.txt", "rt");
    if (archivo == NULL) {
        printf("No se encontro el Archivo\n");
        exit(1);
    }

    char caracter = fgetc(archivo);
    while (!feof(archivo)) {
        printf("%c", caracter);
        caracter = fgetc(archivo);
    }
    fclose(archivo);
}

// Función para buscar una palabra en el archivo usando memoria dinámica
bool encontrar(char *palabra) {
    FILE *archivo = fopen("reservadas.txt", "rt");
    if (archivo == NULL) {
        printf("No se encontro Archivo\n");
        exit(1);
    }

    // Determinar tamaño del archivo
    fseek(archivo, 0, SEEK_END);
    long tamaño = ftell(archivo);
    fseek(archivo, 0, SEEK_SET);

    // Reservar memoria dinámica para el contenido
    char *contenido = (char*)malloc(tamaño + 1);
    if (contenido == NULL) {
        printf("Error de memoria\n");
        fclose(archivo);
        return false;
    }

    // Leer todo el contenido del archivo
    size_t bytes_leidos = fread(contenido, 1, tamaño, archivo);
    contenido[bytes_leidos] = '\0';

    fclose(archivo);

    // Buscar la palabra en el contenido
    char *posicion = contenido;
    bool encontrada = false;
    
    while ((posicion = strstr(posicion, palabra)) != NULL) {
        // Verificar que sea una palabra completa (no parte de otra palabra)
        bool inicio_valido = (posicion == contenido) || 
                           (posicion[-1] == ' ' || posicion[-1] == '\n' || posicion[-1] == '\t');
        
        bool fin_valido = (posicion[strlen(palabra)] == '\0') ||
                         (posicion[strlen(palabra)] == ' ' || 
                          posicion[strlen(palabra)] == '\n' || 
                          posicion[strlen(palabra)] == '\t');
        
        if (inicio_valido && fin_valido) {
            encontrada = true;
            break;
        }
        posicion++; // Continuar buscando
    }

    // Liberar memoria dinámica
    free(contenido);
    return encontrada;
}

// Función mejorada que también devuelve la línea donde se encontró
char* encontrar_y_traducir(char *palabra_ingles) {
    // Mapeo de palabras inglés -> español
    const char *palabras_ingles[] = {
        "return", "break", "int", "float", "case", "const", "continue", NULL
    };
    
    const char *palabras_espanol[] = {
        "retornar", "romper/salir", "entero", "flotante", "caso", "constante", "continuar"
    };

    // Buscar en el array local primero
    for (int i = 0; palabras_ingles[i] != NULL; i++) {
        if (strcmp(palabra_ingles, palabras_ingles[i]) == 0) {
            return strdup(palabras_espanol[i]); // strdup usa malloc internamente
        }
    }

    // Si no se encuentra en el array local, buscar en el archivo
    FILE *archivo = fopen("reservadas.txt", "rt");
    if (archivo == NULL) {
        return NULL;
    }

    char *linea = NULL;
    size_t tamaño_linea = 0;
    ssize_t read;
    char *traduccion = NULL;

    // Leer línea por línea usando memoria dinámica
    while ((read = getline(&linea, &tamaño_linea, archivo)) != -1) {
        // Buscar la palabra en esta línea
        if (strstr(linea, palabra_ingles) != NULL) {
            // Eliminar el salto de línea si existe
            linea[strcspn(linea, "\n")] = 0;
            traduccion = strdup(linea); // Duplicar la línea encontrada
            break;
        }
    }

    // Liberar memoria
    free(linea);
    fclose(archivo);
    
    return traduccion;
}

int main(int argc, char *argv[]) {
    // Verificar número de argumentos
    if (argc < 2) {
        printf("Numero de argumentos Invalidos\n");
        printf("Uso: %s <palabra_a_buscar>\n", argv[0]);
        return 1;
    }

    printf("Buscando: '%s'\n", argv[1]);

    // Método 1: Búsqueda simple
    if (encontrar(argv[1])) {
        printf("La palabra '%s' SI esta en el archivo.\n", argv[1]);
    } else {
        printf("La palabra '%s' NO esta en el archivo.\n", argv[1]);
    }

    // Método 2: Búsqueda con traducción
    char *traduccion = encontrar_y_traducir(argv[1]);
    if (traduccion != NULL) {
        printf("Traduccion encontrada: %s\n", traduccion);
        free(traduccion); // Liberar memoria asignada por strdup
    } else {
        printf("No se encontro traduccion para: %s\n", argv[1]);
    }

    printf("\nContenido del archivo reservadas.txt:\n");
    leer();

    return 0;
}
