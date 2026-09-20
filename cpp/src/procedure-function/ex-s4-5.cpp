#include <iostream>
#include <math.h>
using namespace std;

#define PI 3.14159265

void judul ();

int main (){

    double rad;
    int sudut = 10;

    while (sudut > 0){
        judul();
        cout << "Masukkan sudut (0 untuk berhenti): ";
        cin >> sudut;
        if (sudut == 0) break;
        rad = sudut * (PI/180);
        cout << "Sinus : " << sin(rad) << endl;        
        cout << "Cosinus: " << cos(rad) << endl;
        cout << "Tangen : " << tan(rad) << endl;
        cout << endl;
    }
        cout << "Keluar program"<< endl;
}

void judul(){
    char bintang[]="*****************************";
    cout << bintang << endl;
    cout <<"Hitung sinus, cosinus, tangan" << endl;
    cout << "dengan built-in function C++" << endl;
    cout << "bintang" << endl << endl;

}