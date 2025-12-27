# Shadowd Native Filesystem Modul
Este módulo nativo implementa el sistema de archivos interno del proyecto Shadowd, una base de datos diseñada para escalar a volúmenes extremadamente grandes mediante una arquitectura jerárquica de múltiples capas. El diseño se basa en bloques pequeños y altamente encadenados que permiten alcanzar capacidades de archivo y de disco masivas 

# Arquitectura del sistema
Shadowd utiliza una estructura de seis capas (cinco internas + la capa de archivo) donde cada capa contiene un número invariable de nodos. Cada capa se compone de 31 nodos, lo que permite alcanzar:
  - Capacidad de archivo: 31⁶ bytes de direccionamiento efectivo

# Bloque
Los bloques tienen un tamaño fijo, de 283 bytes.
Cada bloque reserva 4 bytes para datos internos, lo que deja 279 bytes útiles.
Un puntero interno ocupa 9 bytes, por lo que: 279 / 9 = 31 punteros por bloque, cabe aclarar que el primer bytes es el numero de disco y los 8 restantes son efectivamemte el puntero
31 es el límite estándar por capa.
Shadowd usa un sistema fractal, tipo grafo, para navegacion se usa un cursor de 47 bytes, esto permite que un archivo teoricamente pueda superar el YiB sin complicaciones de punteros

# Objetivo del módulo
El módulo representa y gestiona el sistema de archivos interno de Shadowd, incluyendo:
  - Administración de bloques y sus punteros
  - Manejo y descripción de metadato
  - Organización jerárquica del árbol de capa
  - Fundamento estructural para el motor de datos de gran escala
  
  # Estructura del proyecto
    src/
      Algoritm/
        Btree.cpp
        BtreeData.hpp
        Cursors.hpp
        ListaFlexible.hpp
      Block/
        AllocatorBlocks.hpp
        AllocatorBlocks.cpp
        BaseShadowdBlock.hpp
        BaseShadowdBlock.hpp.txt
        BaseShadowdBlock.cpp
        BaseShadowdBlock.cpp.txt
        InsertsHelpers.hpp
        SB.hpp
        ShadowdBlock.hpp
      Disk/
        Disk.cpp
        Disk.hpp
        Metadata.cpp 
        Metadata.hpp  
      Files/
        ShadowdFile.hpp
      Helpers/
        BigEndianCover.hpo
        RandomAccessFile.hpp
        ...
# Estado del proyecto
Actualmente en etapa temprana de desarrollo.
La API, el formato interno y la estructura jerárquica pueden sufrir modificaciones mientras se terminan de definir los estándares del sistema completo.

Decisiones estables
  - Bloques de 283 bytes fijos
  - Grafo de grado 31 por nodo
  
# Requisitos
  - C++15 o superior
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
- Herramientas de visualización del árbol entero
- Mejoras en los algoritmos de transporte

# Licencia
  Apache 2.0
