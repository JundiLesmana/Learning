#include <iostream>
#include <iomanip>
using namespace std;

double bunga = 6.0;

void HitungBunga(double saldo[]);

int main(){

    int    akun[5]  = {101, 102, 103, 104, 105};
    double saldo[5] = {1000000, 2000000, 1500000, 3000000, 2500000};

    cout << fixed << setprecision(0);

    cout << "===============================================\n";
    cout << "   HITUNG BUNGA SETAHUN (bunga = 6%)  \n";
    cout << "===============================================\n\n";

    cout << "--- AKUN & SALDO (awal) ---\n";
    for (int i = 0; i < 5; i++){
        cout << "  Akun: " << akun[i]
             << "   Saldo: " << saldo[i] << endl;
    }

    HitungBunga(saldo);

    cout << "\n--- AKUN & SALDO BARU (setelah HitungBunga) ---\n";
    for (int i = 0; i < 5; i++){
        cout << "  Akun: " << akun[i]
             << "   Saldo Baru: " << saldo[i] << endl;
    }

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void HitungBunga(double saldo[]){

    for (int i = 0; i < 5; i++){
        saldo[i] = saldo[i] * (1 + bunga / 100);
    }
}