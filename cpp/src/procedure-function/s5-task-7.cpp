#include <iostream>
using namespace std;

int pangkat(int x, int y);

int main(){

    int x, y;

    cout << "===============================================\n";
    cout << "   FUNGSI REKURSIF PANGKAT (x pangkat y)\n";
    cout << "===============================================\n\n";

    cout << "Masukkan angka x: ";
    cin >> x;
    cout << "Masukkan pangkat y: ";
    cin >> y;

    cout << "\n" << x << " pangkat " << y << " adalah " << pangkat(x, y) << endl;

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

int pangkat(int x, int y){

    if (y == 0){
        return 1;               
    }
    return x * pangkat(x, y - 1); 
}