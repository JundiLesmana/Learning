#include <iostream>
using namespace std;

int perkalian (int a, int b);

int main (){

    int z;
    z = perkalian (3,4);
    cout << "Hasil perkalian: " << z << endl;   
}

int perkalian (int a, int b){
    int r = a*b;
    return r;
}