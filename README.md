# Shadowd Native Filesystem Modul
Este módulo nativo implementa el sistema de archivos interno del proyecto Shadowd, una base de datos diseñada para escalar a volúmenes extremadamente grandes mediante una arquitectura jerárquica de múltiples capas. El diseño se basa en bloques pequeños y altamente encadenados que permiten alcanzar capacidades de archivo y de disco masivas sin depender de tamaños de bloque fijos

# Arquitectura del sistema
Shadowd utiliza una estructura de seis capas (cinco internas + la capa de archivo) donde cada capa contiene un número variable de nodos. El valor por defecto es 31 nodos por capa, lo que permite alcanzar:
  - Capacidad de archivo: 31⁶ bytes de direccionamiento efectivo
  - Cantidad total de discos internos: También 31⁶, cada uno representando un espacio de direccionamiento independiente dentro del sistema general

# Bloque
Los bloques no tienen tamaño fijo, pero existe un valor por defecto: 252 bytes.
Cada bloque reserva 4 bytes para datos internos, lo que deja 248 bytes útiles.
Un puntero interno ocupa 8 bytes, por lo que:248 / 8 = 31 punteros por bloque
31 es el límite estándar por capa con los valores por defecto
Si un número excede los 8 bytes de representación, el disco deja de ser válido y el sistema detiene la expansión
Este enfoque permite que Shadowd mantenga bloques pequeños, nodos simples y una expansión casi fractal en profundidad, lo que habilita su capacidad extrema sin recurrir a bloques gigantes.

# Objetivo del módulo
El módulo representa y gestiona el sistema de archivos interno de Shadowd, incluyendo:
  - Administración de bloques y sus punteros
  - Manejo y descripción de metadato
  - Organización jerárquica del árbol de capa
  - Fundamento estructural para el motor de datos de gran escala
  
  # Estructura del proyecto
    src/
      Block/
        AllocatorBlocks.hp
        ...  
      Disk/
        Disk.hpp 
        Metadata.hpp  
        ...
# Estado del proyecto
Actualmente en etapa temprana de desarrollo.
La API, el formato interno y la estructura jerárquica pueden sufrir modificaciones mientras se terminan de definir los estándares del sistema completo.

# Requisitos
  - C++20 o superior
  - Make como sistema de construcción
  - GCC o Clang

# Compilación
``` cpp
make
```
# Próximos pasos
- Implementación completa del sistema de archivos Shadowd
- Validación y límites de expansión
- Mecanismos de integridad y recuperación
- Herramientas de visualización del árbol intero

# Licencia
  Apache 2.0
