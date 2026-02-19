#include "Disk.hpp"
#include "Metadata.hpp"

/*El siguiete codigo es generado por inteligencia artificial
 *Esto porque aun no domino todos los includes de c++
 */
#include <filesystem>
#include <fstream>

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

bool copiarArchivo(const fs::path& origen, const fs::path& destino) {
    try {
        fs::copy_file(
            origen,
            destino,
            fs::copy_options::overwrite_existing   // reemplaza si ya existe
        );
        return true;
    } catch (const std::exception& e) {
        return false;
    }
}

bool borrarArchivo(const fs::path& archivo){
    if(archivoExiste(archivo)){
        return fs::remove(archivo);
    }
    return true;
}

/*Fin de codigo generado*/

Disk::Disk(std::string pathVar, int blockSizeVar)
    : meta(blockSizeVar, pathVar), 
    alloc(meta),
    rootDirectory(alloc, 2)
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
        rootDirectory = meta.make(alloc);
    }

    //si ya existia uno entonces
    else{
        rootDirectory = meta.load(alloc);
    }
}

void Disk::persist(){
    meta.persist(rootDirectory);
}

bool Disk::backup(){
    fs::path archivo(meta.path);
    fs::path back(meta.path+".back");

    return copiarArchivo(archivo, back);
}

bool Disk::restore(int intent){
    fs::path archivo(meta.path);
    fs::path back(meta.path+".back");

    if(archivoExiste(back)){
        if(copiarArchivo(back, archivo)){
           borrarArchivo(back);
        }else{
            if(intent <= 10){
                return restore(++intent);
            }
            borrarArchivo(back);
            borrarArchivo(archivo);
        }
    }else{
        borrarArchivo(archivo);
    }
    
    if(archivoExiste(archivo)){
        rootDirectory = meta.load(alloc);
    }else{
        rootDirectory = meta.make(alloc);
    }
    return true;
}