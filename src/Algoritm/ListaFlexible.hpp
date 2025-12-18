#include <cstddef>
#include <vector>

template<typename T>
class ListaFlexible {
    private:
        std::vector<T> datos;

    public:
        // Inserta el valor en la posición dada
        void insertar(size_t pos, const T& valor) {
            if (pos >= datos.size()) {
                // Si la posición está más allá del tamaño, solo añadimos al final
                datos.resize(pos + 1); // rellenamos con valores por defecto
            }
            datos.insert(datos.begin() + pos, valor); // inserta y empuja elementos hacia adelante
        }

        // Obtener tamaño
        size_t tamaño() const {
            return datos.size();
        }

        // Obtener una instancia
        T& get(size_t pos){
            return datos[pos];
        }
};
