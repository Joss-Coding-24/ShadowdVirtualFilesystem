#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>
#include "../Helpers/BigEndianCover.hpp"

struct Cursor1{
    size_t pos = 0, off = 0;
};
struct Cursor2{
    size_t pos = 0, off = 0;
    Cursor1 root;
};
struct Cursor3{
    size_t pos = 0, off = 0;
    Cursor2 root;
};
struct Cursor4{
    size_t pos = 0, off = 0;
    Cursor3 root;
};
struct Cursor5{
    size_t pos = 0, off = 0;
    Cursor4 root;
};
using bytes = std::vector<uint8_t>;
const int pos_size = 5; // 5 bytes para posicion
const int cur_size = 80; //80 bytes para un cursor completo

class Cursor{
    public:
        Cursor(bytes cursorBytes)
        {
            size_t layer=6, cx = 1;
            for(size_t i = 0; i < cursorBytes.size(); i += pos_size){
                bytes buff;
                buff.insert(buff.begin(), cursorBytes.begin()+i, cursorBytes.begin()+i+pos_size);
                size_t pos = beToSize(buff, pos_size);
                if(layer == 6){
                    layer6.pos = pos;
                    layer6.off = pos*11;
                    layer = 5; cx = 5;
                }else if(layer == 5){
                    if(cx == 5){
                        layer5.pos = pos;
                        layer5.off = pos*11;
                        cx = 4;
                    }else if(cx == 4){
                        layer5.root.pos = pos;
                        layer5.root.off = pos*11;
                        cx = 3;
                    }else if(cx == 3){
                        layer5.root.root.pos = pos;
                        layer5.root.root.off = pos*11;
                        cx = 2;
                    }else if(cx == 2){
                        layer5.root.root.root.pos = pos;
                        layer5.root.root.root.off = pos*11;
                        cx = 1;
                    }else if(cx == 1){
                        layer5.root.root.root.root.pos = pos;
                        layer5.root.root.root.root.off = pos*11;
                        layer = 4;
                        cx = 4;
                    }
                }else if(layer == 4){
                    if(cx == 4){
                        layer4.pos = pos;
                        layer4.off = pos*11;
                        cx = 3;
                    }else if(cx == 3){
                        layer4.root.pos = pos;
                        layer4.root.off = pos*11;
                        cx = 2;
                    }else if(cx == 2){
                        layer4.root.root.pos = pos;
                        layer4.root.root.off = pos*11;
                        cx = 1;
                    }else if(cx == 1){
                        layer4.root.root.root.pos = pos;
                        layer4.root.root.root.off = pos*11;
                        layer = 3;
                        cx = 3;
                    }
                }else if(layer == 3){
                    if(cx == 3){
                        layer3.pos = pos;
                        layer3.off = pos*11;
                        cx = 2;
                    }else if(cx == 2){
                        layer3.root.pos = pos;
                        layer3.root.off = pos*11;
                        cx = 1;
                    }else if(cx == 1){
                        layer3.root.root.pos = pos;
                        layer3.root.root.off = pos*11;
                        cx = 2;
                        layer = 2;
                    }
                }else if(layer == 2){
                    if(cx == 2){
                        layer2.pos = pos;
                        layer2.off = pos*11;
                        cx = 1;
                    }else if(cx == 1){
                        layer2.root.pos = pos;
                        layer2.root.off = pos*11;
                        layer = 1;
                    }
                }else if(layer==1) {
                    layer1.pos = pos;
                    layer1.off = pos* 11;                
                }
            }
        }
        size_t getActuallyCNOff(size_t layer);
        size_t getActuallyCNPos(size_t layer);
        void setPunterLevel(size_t layer);
        bytes toBytes();
    private:
        
        Cursor1 layer6;
        Cursor5 layer5;
        Cursor4 layer4;
        Cursor3 layer3;
        Cursor2 layer2;
        Cursor1 layer1;
};