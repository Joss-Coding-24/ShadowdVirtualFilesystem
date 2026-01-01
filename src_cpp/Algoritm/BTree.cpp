#include "BTree.hpp"
#include <cassert>

ProccessResponce resolveBetween(ProccessParameter& pp, BTreeData& actual, Actions action, States state){
     //si se contradice enconces va en medio de esas
    ProccessResponce res;
    BTreeData& before = *pp.ant; // por el momento queda asi poeque no.me permite hacer "=" entre los dos punteros
    if(action == Actions::INSERT_BEFORE && state == States::NEXT){
        res.before = std::make_unique<BTreeData>(actual);
        res.next = std::make_unique<BTreeData>(before);
    }else {
        res.before = std::make_unique<BTreeData>(before);
        res.next = std::make_unique<BTreeData>(actual);
    }
    res.action = action;
    res.state = state;
    return res;
}

ProccessResponce BTree::resolveDescend(size_t newPos, States newState, BTreeData& actual, std::string& name){
    ProccessParameter params;
    params.pos = newPos;
    params.state = newState;
    params.ant = std::make_unique<BTreeData>(actual);
    return proccess(params, name);
}

ProccessResponce BTree::proccess(ProccessParameter& pp, std::string& newNodeName){
    BTreeData& actual = list.get(pp.pos);
    CmpStr cmp = compareStrings(actual.getName(), newNodeName);
    if(cmp == CmpStr::LESS){
        if(pp.state == States::UNKNOWN || pp.state == States::BEFORE){
            if(pp.pos==0){
                if(loadAndAppendNode(actual, Punter::PUNTER_BEFORE, false)){
                    pp.pos++;
                    return resolveDescend(0, States::BEFORE, actual, newNodeName);
                }else{
                    ProccessResponce res;
                    res.before = std::make_unique<BTreeData>(actual);
                    res.next = nullptr; //no existe un siguiente;
                    res.action = Actions::INSERT_FIRST;
                    res.state = States::BEFORE;
                    return res;
                }
            }
            //si es menor y aun no estamos al final, simplemente continuamos bajando 
            return resolveDescend(pp.pos-1, States::BEFORE, actual, newNodeName);
        }else if (pp.state == States::NEXT) {
            return resolveBetween(pp, actual, Actions::INSERT_BEFORE, States::NEXT);
        }else if (pp.state == States::EQUALS){
            return resolveBetween(pp, actual, Actions::NONE, States::EQUALS);
        }
    } else if(cmp == CmpStr::EQUAL){
        return resolveBetween(pp, actual, Actions::NONE, States::EQUALS);
    }else if(cmp == CmpStr::GREATER){
        if(pp.state == States::UNKNOWN || pp.state == States::NEXT){
            if(pp.pos==list.tamaño()){
                if(loadAndAppendNode(actual, Punter::PUNTER_NEXT, true)){
                    ProccessParameter params;
                    params.pos = list.tamaño()-1;
                    params.state = States::NEXT;
                    params.ant = std::make_unique<BTreeData>(actual);
                    return proccess(params, newNodeName);
                }else{
                    ProccessResponce res;
                    res.before = std::make_unique<BTreeData>(actual);
                    res.next = nullptr; //no existe un siguiente;
                    res.action = Actions::INSERT_LAST;
                    res.state = States::NEXT;
                    return res;
                }
            }
            ProccessParameter params;
            params.pos = pp.pos+1;
            params.state = States::NEXT;
            params.ant = std::make_unique<BTreeData>(actual);
            return proccess(params, newNodeName);
        }else if (pp.state == States::BEFORE) {
            return resolveBetween(pp, actual, Actions::INSERT_NEXT, States::BEFORE);
        }else if (pp.state == States::EQUALS) {
            return resolveBetween(pp, actual, Actions::NONE, States::EQUALS);
        }
    }
    assert(false && "Código inalcanzable en proccess");
    return {}; // solo para el compilador
}