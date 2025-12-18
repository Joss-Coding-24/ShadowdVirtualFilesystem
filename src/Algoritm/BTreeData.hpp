#include "../Block/BaseShadowdBlock.hpp"
#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <string>
#include <vector>
#include "../Helpers/BigEndianCover.hpp"

class BTreeData{
    public:
        explicit BTreeData(size_t posVar, Alloc& alloc) :
            block(posVar, alloc),
            pos(posVar)
        {
            caps.resize(    7, -1);
        }

        const size_t pos;

        std::string getName(){
            if(nameLoaded) readName();
            return name;
        }

        size_t getPunter(bool punter){
            if(punter){
                if(next == 0) readPunter(punter);
                return next;
            }else{
                if(before == 0) readPunter(punter);
                return before;
            }
        }

        int getLayer(int layer){
            if(layer < 0 || layer > 6) return -1;
            if(caps[layer] < 0) readLayer_(layer);
            return caps[layer];
        }

        void setLayer(int layer, int value){
            if(value < 0 || value > 31 || caps[layer] == value) return;
            caps[layer] = value;
        }

        void setPointer(bool punter, size_t value){
            if(value <= 0 || value > std::numeric_limits<uint64_t>::max()) return;
            if(punter){
                next = value;
            }else{
                before = value;
            }
        }

        void setName(std::string newName){
            nameLoaded = true;
            name = newName;
        }

        void persist(){
            std::vector<uint8_t> data, dataN, dataP, dataC;

            dataN = writeName();
            data.insert(data.end(), dataN.begin(), dataN.end());

            dataP = writePunters();
            data.insert(data.end(), dataP.begin(), dataP.end());

            dataC = writeCaps();
            data.insert(data.end(), dataC.begin(), dataC.end());

            block.clearLoteBlock();
            block.writeBlock(data);
            block.writeIntern();
        }
    private:
        bSB block;
        std::vector<size_t> caps;
        size_t next = 0;
        size_t before = 0;
        std::string name;
        bool nameLoaded = false;
        void readLayer_(int layer){
            std::vector<uint8_t> bytes = block.readTo(242+layer, 243+layer);
            caps[layer] = beToSize(bytes, 1);
        }
        
        void readPunter(bool punter){
            if(punter){
                std::vector<uint8_t> bytes = block.readTo(226, 233);
                next = beToSize(bytes, 8);
            }else{
                std::vector<uint8_t> bytes = block.readTo(234, 241);
                before =beToInt(bytes, 8);
            }
        }

        void readName(){
            std::vector<uint8_t> bytes = block.readTo(0, 225);
            auto it = std::find(bytes.begin(), bytes.end(), '\0');
            std::string texto(bytes.begin(), it);
            name = texto;
            nameLoaded = true;
        }
        
        std::vector<uint8_t> writeName(){
            std::size_t N = 255;
            std::vector<uint8_t> buffer(N, '\0');
            std::size_t len = std::min(name.size(), N);
            std::copy_n(name.begin(), len, buffer.begin());
            return buffer;
        }

        std::vector<uint8_t> writePunters(){
            std::vector<uint8_t> buffer, bytesN, bytesB;
            bytesN = sizeToBe(next, 8);
            bytesB = sizeToBe(before, 8);
            buffer.insert(buffer.end(), bytesN.begin(), bytesN.end());
            buffer.insert(buffer.end(), bytesB.begin(), bytesB.end());
            return buffer;
        }

        std::vector<uint8_t> writeCaps(){
            std::vector<uint8_t> buffer;
            for(int i = 0; i < 7; i++){
                std::vector<uint8_t> bytes = intToBe(caps[i], 1);
                buffer.insert(buffer.end(), bytes.begin(), bytes.end());
            }
            return buffer;
        }
};