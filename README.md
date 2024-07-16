# Trabajo Práctico 1
## Pedro Gallino 107587

### Implementación

* Se setea el número de workers
* Se paraleliza el procesamiento de archivos. -> par_iter()
* Se paraleliza el procesamiento de las lineas dentro de cada archivo. -> par_bridge()
* Se reduce todo en una Struct llamada ResultData. -> crate serde_json.
* ResultData se tranforma en String Json y se imprime por stdout.

### Archivos
 * main.rs
 * processors.rs contiene todas las funciones que procesan los archivos.
 * structs.rs contiene todas las structs necesarias para el procesamiento de los archivos.

### Resultados

* Se procesan todos los archivos en aproximadamente 2 minutos utilizando 8 workers en mi CPU de 4 núcleos con dos workers por núcleo.

  ![image](https://github.com/concurrentes-fiuba/2024-1c-tp1-pgallino/assets/90009211/bc913f00-3651-41cd-8bcb-14d0bf5a8047)

  ![image](https://github.com/concurrentes-fiuba/2024-1c-tp1-pgallino/assets/90009211/ec787cc8-a76e-4333-8ab1-b0f1f1e10bd6)



[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/VqwN-ppG)
