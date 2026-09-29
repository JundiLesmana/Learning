#include <iostream>
#include <fstream>
using namespace std;

#define MAX 100

void cari4(int data1[], int jml, int d, int *dx);

int main (){
    system("cls");
    int data1[MAX];
    int jml = 0, d, dx;
    string arsip = "src/searching/data-s6-3.txt";

    ifstream filein(arsip);
    if (!filein){
        cout << "Arsip " << arsip << " tidak bisa dibuka!" << endl;
        return 1;
    }
    while (jml < MAX && filein >> data1[jml]){
        jml++;
    }
    filein.close();

    cout << "Data dari arsip (" << arsip << "): ";
    for (int k=0; k<jml; k++){
        cout << data1[k] << " ";
    }
    cout << endl;

    cout << "Masukkan data yang dicari: ";
    cin >> d;

    cari4(data1, jml, d, &dx);

    if (dx != -1){
        cout << "Data " << d << " ditemukan pada indeks ke " << dx << endl;
    } else {
        cout << "Data " << d << " tidak ada di dalam arsip." << endl;
    }

    return 0;
}

void cari4(int data1[], int jml, int d, int *dx){
    int k = 0;
    while (k < jml && data1[k] != d){
        k++;
    }
    if (k < jml){
        *dx = k;                
    } else {
        *dx = -1;
    }
}