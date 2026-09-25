#include <iostream>
using namespace std;

int angka=50;

void f1(){
    
    cout << "Dalam f1, angka= " << angka << endl;
}

int main (){
    cout << "Dalam main, angka= " << angka << endl;
    f1();
    cout << "Dalam main, angka= " << angka << endl;
}