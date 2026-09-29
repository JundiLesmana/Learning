#include <iostream>
using namespace std;

void cari1(int data1[], int jml, int d, int *dx);

int main (){

    system("cls");
    int data1[]={10,2,12,3,30,100};
    int d, k, jml=6; int dx;
    cout << "Elemen array: ";
    for (k=0; k<jml; k++){
        cout << data1[k] << " "; cout << endl;
        cari1(data1, jml, d, &dx);
        cari1 (data1, jml, d, &dx);
        if(dx!=-1){
            cout << "Data yang dicari tidak ada pada indeks ke " << dx << endl;
        } else {
            cout << "Data ditemukan" << endl;
        }
    }
}

void cari1(int data1[], int jml, int d, int *dx){
    int k=0;
    while(k<jml && data1[k]!=d)k++;
    if (data1[k]==d){
        *dx=k;
    } else {
        *dx=-1;
    }
    }