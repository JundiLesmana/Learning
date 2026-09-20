#include <iostream>
#include <time.h>
#include <cstdio>
using namespace std;
#define JML_DATA 50

void isi_random(int a[]);

void cetak (int a[]);
void cari (int a[], int x);

int main(){
    
    int angka[JML_DATA];
    isi_random(angka);
    cetak(angka);
    int n = 10;
    while (n>=0){
    cout <<endl
    << "Masukkan angka (0 untuk berhenti): ";
    
    cin >> n;
    if (n<0) break ;
    cari (angka, n);
    }
}

void isi_random(int a[]){
    srand(time(NULL));
    for (int i=0; i<JML_DATA; i++){
        a[i] = rand()%100;
    }
}

void cetak (int a[]){
char garis[]="--------------------------------------------------";
    cout << garis;
    for (int i=0; i<JML_DATA; i++) {
        if (i%10==0) cout << endl;
        printf("%02d ", a[i]);
    }
    cout<<endl<<garis<<endl;
}

void cari(int a[], int x) {
    int i;
    for (i=0; i<JML_DATA; i++) {
        if (a[i]==x) {
            cout << "Angka " << x << " ditemukan pada indeks ke-" << i << endl;
            break;
        }
    }

    if (i==JML_DATA)
    cout << "Angka tidak ditemukan"<<endl;
}