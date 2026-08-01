# Graficas-Lab2
# Conway's Game of Life
# Hugo Méndez Lee - 241265

Una implementación completa y psicodélica del clásico autómata celular **"El Juego de la Vida"** de John Horton Conway, escrito en Rust y renderizado desde cero en un _framebuffer_ personalizado utilizando la librería `minifb`.

## 📸 Demo

<img width="800" height="600" alt="Conways game of life" src="https://github.com/user-attachments/assets/d337e6c7-f819-4ed7-921e-d57fe7dd73c9" />

## 🚀 Características Principales

* **Renderizado a Mano:** Todo el juego se dibuja utilizando única y exclusivamente una función `point(x, y)` implementada en el `Framebuffer`. No se utilizaron librerías de gráficos avanzadas ni funciones geométricas pre-construidas.
* **Modo Psicodélico Neón:** En lugar de los tradicionales blanco y negro, esta implementación incluye un motor de color dinámico. Cada célula viva adquiere colores neón brillantes basados en su posición en la cuadrícula y en el número de generación (frame) actual.
* **Universo Toroidal:** El universo del juego no tiene bordes finitos. Si una nave o estructura cruza el límite de la pantalla, aparecerá mágicamente por el lado opuesto (se comporta como un toroide o dona).
* **Resolución Escalable (Responsivo):** Todos los organismos iniciales están programados para distribuirse utilizando porcentajes respecto al tamaño del lienzo. Si cambias la resolución a 100x100, 300x300, etc., el ecosistema se adaptará automáticamente a la nueva escala.

## 🖥️ Especificaciones Técnicas

* **Librería de Gráficos:** [`minifb`](https://crates.io/crates/minifb) (v0.28.0) para la creación de la ventana y la escritura directa en el buffer de memoria.
* **Resolución de la Ventana:** 800x600 píxeles.
* **Resolución del Framebuffer (Lienzo Interno):** 125x125 píxeles (Escalado para dar un efecto retro o pixel-art).
* **Tasa de Actualización:** Configurada con un delay de `100ms` por ciclo para una animación veloz y fluida.
* **Lenguaje:** Rust (Edición 2024).
## 🦠 Ecosistema Incluido

El patrón inicial contiene una cuidadosa y muy variada selección de organismos distribuidos por el mapa:

### Estáticos (Vidas Inmóviles)
* Block (Bloque)
* Beehive (Colmena)
* Loaf (Hogaza)
* Boat (Barco)

### Osciladores
* Blinker (Parpadeador)
* Toad (Sapo)
* Beacon (Faro)
* Pulsar (Pulsar Gigante)

### Naves Espaciales (Spaceships)
* Glider (Planeador)
* LWSS (Lightweight spaceship - Nave Ligera)
* MWSS (Middleweight spaceship - Nave Mediana)

### Estructuras Complejas Avanzadas (Matusalenes)
Para evitar el tan usado _Gosper Glider Gun_, este proyecto incluye dos famosos Matusalenes que demuestran la complejidad del caos emergente a partir de unas pocas células:
1. **Acorn (La Bellota):** Una diminuta estructura de 7 células que tarda **5,206 generaciones** en estabilizarse, creando una explosión masiva por todo el mapa.
2. **Diehard (El Duro de Matar):** Una estructura en apariencia estable que interactúa de formas inesperadas hasta desvanecerse por completo exactamente en la generación 130.

## 🛠 Instalación y Ejecución

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/). 

1. Clona este repositorio o abre la carpeta del proyecto en tu terminal.
2. Compila y ejecuta el código con el siguiente comando:

```bash
cargo run --release
```

Presiona `ESC` o cierra la ventana para salir de la simulación.
