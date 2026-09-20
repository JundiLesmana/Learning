#include <iostream>
using namespace std;

void f1(){

    int angka = 44;
    cout << "Dalam f1, angka= " << angka << endl;
}

int main (){

    int angka =35;

    cout << "Dalam main, angka= " << angka << endl;

    f1();
    cout << "Dalam main, angka= " << angka << endl;
}