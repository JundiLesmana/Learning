#include <iostream>
#include <iomanip>
using namespace std;

const double PI = 3.14159265;

double luas(double r);       
double keliling(double r);   

int main(){

    double r;

    cout << "===============================================\n";
    cout << "   LUAS & KELILING LINGKARAN (PI global)\n";
    cout << "===============================================\n\n";

    cout << "Masukkan jari-jari lingkaran: ";
    cin >> r;

    cout << "\n  Luas lingkaran     = " << fixed << setprecision(6) << luas(r)     << endl;
    cout << "  Keliling lingkaran = " << keliling(r) << endl;

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

double luas(double r){
    return PI * r * r;
}

double keliling(double r){
    return 2 * PI * r;
}