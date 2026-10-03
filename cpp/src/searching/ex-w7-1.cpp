#include <iostream>
#include <cstdlib>
#define jmldt 6
using namespace std;

void cari4(int data4 [], int jml, int cari, int *dx);

int main (){
    int data4[jmldt] = {60, 50, 40, 30, 20, 10},cari,dx,awal;

    cout <<"Elemen Array: ";
    for(int awal=0;awal<jmldt;awal++)
    cout<<data4[awal]<<" ";cout<<endl;
    cout << "Masukan data yang akan dicari ?"; cin >> cari;
    cari4(data4, jmldt, cari, &dx);

    if (dx!=-1)
    cout << "Data ditemukan pada indeks ke " << dx << endl;
    else cout << "Data tidak ditemukan" << endl;
}

void cari4 (int data4 [], int jml, int cari, int *dx){
    
    bool ketemu = false;
    int akhir = jml-1, awal =0,c;
    while (awal<=akhir && !ketemu)

{
    c=(awal+akhir)/2;
    if (data4[c]==cari) ketemu = true;
    else if (data4[c]<cari) awal = c-1;
    else akhir = c+1;
}

    if (ketemu) *dx = c;
    else *dx = -1;
}


