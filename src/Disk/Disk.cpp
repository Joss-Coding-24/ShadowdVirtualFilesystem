#include "Disk.hpp"

/*El siguiete codigo es generado por inteligencia artificial
 *Esto porque aun no domino todos los includes de c++
 */
#include <filesystem>
#include <fstream>
#include <iostream>

namespace fs = std::filesystem;

bool carpetaExiste(const fs::path& carpeta) {
    return fs::exists(carpeta) && fs::is_directory(carpeta);
}

bool archivoExiste(const fs::path& archivo) {
    return fs::exists(archivo) && fs::is_regular_file(archivo);
}

void crearCarpeta(const fs::path& carpeta) {
    fs::create_directories(carpeta);
}

void crearArchivo(const fs::path& archivo) {
    std::ofstream f(archivo);
}
/*Fin de codigo generado*/

Disk::Disk(std::string pathVar, int blockSizeVar)
    : meta(blockSizeVar, pathVar), alloc(meta)
{
    //definimoa los paths al disco virtual
    fs::path archivo(pathVar);
    fs::path parent = archivo.parent_path();

    //verificamos que la carpeta del diaco exista
    if(!carpetaExiste(parent)){
        //si no exsite la creamos
        crearCarpeta(parent);
    }

    //verificamos si el disco existe
    if(!archivoExiste(archivo)){
        //si mo existe creamos uno nuevo
        crearArchivo(archivo);
        rootDirectory = meta.make();
    }

    //si ya existia uno entonces
    else{
        rootDirectory = meta.load();
    }
}