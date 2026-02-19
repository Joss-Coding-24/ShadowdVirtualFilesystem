#include "Metadata.hpp"
#include "../Block/AllocatorBlocks.hpp"

Metadata::Metadata(int blockSizeVar, std::string pathVar){
    sizeBlock = blockSizeVar;
    path = pathVar;
}

SDirectory Metadata::load(AllocatorBlocks& allocs){
    SDirectory root(allocs, 2);
    return root;
}

SDirectory Metadata::make(AllocatorBlocks& allocs){
    SDirectory root(allocs, 2);
    return root;
}
void Metadata::persist(SDirectory dir){
    dir.persist();
    dir.updateTree();
}
