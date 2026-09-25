#include <iostream>
using namespace std;

void swap(int *a, int *b);

int main(){

    /* --- Variabel LOKAL --- */
    int x, y;

    cout << "===============================================\n";
    cout << "   MENUKAR NILAI DUA VARIABEL (SWAP)\n";
    cout << "===============================================\n\n";

    cout << "Masukkan nilai variabel pertama (x): ";
    cin >> x;
    cout << "Masukkan nilai variabel kedua  (y): ";
    cin >> y;

    cout << "\n--- ANTES MENUKAR (awal) ---\n";
    cout << "  x = " << x << "    y = " << y << endl;

    swap(&x, &y);

    cout << "\n--- DEPOIS MENUKAR (hasil) ---\n";
    cout << "  x = " << x << "    y = " << y << endl;

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void swap(int *a, int *b){

    int temp = *a;
    *a = *b;
    *b = temp;
}