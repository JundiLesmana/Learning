#include <iostream>
using namespace std;

void cari2(int data1[], int jml, int d, int *dx);

int main (){
    system("cls");
    int data1[]={10,2,12,3,30,100};
    int d, jml=6, dx;

    cout << "Elemen array: ";
    for (int k=0; k<jml; k++){
        cout << data1[k] << " ";
    }
    cout << endl << endl;

    cout << "Masukkan data yang dicari: ";
    cin >> d;

    cari2(data1, jml, d, &dx);

    if (dx != -1){
        cout << "Data ditemukan pada indeks ke " << dx << endl;
    } else {
        cout << "Data " << d << " tidak ada di dalam array." << endl;
    }

    return 0;
}

void cari2(int data1[], int jml, int d, int *dx){
    int k = jml - 1;                 // mulai dari indeks terbesar
    while (k >= 0 && data1[k] != d){ // berger ke indeks terkecil
        k--;
    }
    if (k >= 0){
        *dx = k;                     // data ditemukan di indeks k
    } else {
        *dx = -1;                    // data tidak ada
    }
}