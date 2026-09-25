#include <iostream>
using namespace std;

void deretGenap(int n);

int main(){

    int n;

    cout << "===============================================\n";
    cout << "   DERET BILANGAN GENAP (REKURSIF)\n";
    cout << "===============================================\n\n";

    cout << "Masukkan jumlah bilangan genap: ";
    cin >> n;

    cout << "\n" << n << " bilangan genap: ";
    deretGenap(n);         
    cout << endl;

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void deretGenap(int n){

    if (n == 0){
        return;              
    }

    cout << 2 * n;           

    if (n > 1){
        cout << " ";         
    }

    deretGenap(n - 1);     
}