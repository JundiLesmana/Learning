#include <iostream>
using namespace std;

void cari3(int data1[], int jml, int d, int *dx);

int main (){
    system("cls");
    int data1[]={22,45,32,11,20,55,70};
    int d = 55, jml = 7, dx;

    cout << "Data: ";
    for (int k=0; k<jml; k++){
        cout << data1[k] << " ";
    }
    cout << endl;
    cout << "Data yang dicari: " << d << endl << endl;

    cout << "--- PROSES PENCARIAN ---" << endl;
    cari3(data1, jml, d, &dx);

    if (dx != -1){
        cout << endl << "Data " << d << " ditemukan pada indeks ke " << dx << endl;
    } else {
        cout << endl << "Data " << d << " tidak ada di dalam array." << endl;
    }

    return 0;
}

void cari3(int data1[], int jml, int d, int *dx){
    int k = 0;
    int step = 0;
    while (k < jml && data1[k] != d){
        step++;
        cout << "  [" << step << "] data[" << k << "]=" << data1[k]
             << "  -> " << data1[k] << " != " << d
             << " , data belum ditemukan" << endl;
        k++;
    }
    if (k < jml){
        step++;
        cout << "  [" << step << "] data[" << k << "]=" << data1[k]
             << "  -> " << data1[k] << " == " << d
             << " , DATA DITEMUKAN!" << endl;
        *dx = k;
    } else {
        *dx = -1;
    }
    cout << "  Total membandingkan: " << step << endl;
}