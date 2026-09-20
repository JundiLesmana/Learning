#include <iostream>
using namespace std;

void judul();
int fungsiX(int x);

int main(){

    int angka = -1;
    judul();
    while (angka!=0){

    cout <<"Masukan angka (0 untuk berhenti):";
    cin >> angka;
        
    if (angka ==0) break;
    cout << "Hasil Fungsi: " << fungsiX(angka)<< endl << endl;
    judul();

    }
}

void judul(){
    cout << endl;
    cout << "Program menghitung hasil" << endl;
    cout << " fungsi 3x^2+5*x+4"<<endl;
    cout << endl;
}

int fungsiX(int x){
    return 3*x*x + 5*x + 4;
}