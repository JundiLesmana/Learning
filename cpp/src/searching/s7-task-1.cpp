#include <iostream>
using namespace std;

#define jmldt 6

void cari5(int data4[], int jml, int cari, int *dx);

int main (){
    system("cls");
    int data4[jmldt] = {60, 50, 40, 30, 20, 10}; // data urut descending
    int cari, dx;

    cout << "Elemen Array (descending): ";
    for (int k=0; k<jmldt; k++){
        cout << data4[k] << " ";
    }
    cout << endl << endl;

    cout << "Masukkan data yang akan dicari: ";
    cin >> cari;

    cari5(data4, jmldt, cari, &dx);

    if (dx != -1){
        cout << endl << "Data ditemukan pada indeks ke " << dx << endl;
    } else {
        cout << endl << "Data tidak ditemukan" << endl;
    }

    return 0;
}

void cari5(int data4[], int jml, int cari, int *dx){
    bool ketemu = false;
    int awal = 0, akhir = jml - 1, c, langkah = 0;

    while (awal <= akhir && !ketemu){
        c = (awal + akhir) / 2;            // indeks tengah
        langkah++;
        cout << "  [" << langkah << "] awal=" << awal << " akhir=" << akhir
             << " -> c=" << c << ", data4[" << c << "]=" << data4[c] << endl;

        if (data4[c] == cari){
            ketemu = true;
            cout << "        -> data4[c] == cari : DATA DITEMUKAN!" << endl;
        } else if (data4[c] > cari){
            cout << "        -> data4[c] > cari  : geser ke kanan (awal = c + 1)" << endl;
            awal = c + 1;
        } else {
            cout << "        -> data4[c] < cari  : geser ke kiri (akhir = c - 1)" << endl;
            akhir = c - 1;
        }
    }

    if (ketemu){
        *dx = c; 
    } else {
        *dx = -1;
    }
}