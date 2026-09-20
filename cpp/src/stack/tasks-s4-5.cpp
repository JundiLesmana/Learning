#include <iostream>
using namespace std;

int main() {
    int n = 10; 
    
    cout << "--- LATIHAN 5: Ciri Kondisi Single Stack (n=10) ---" << endl;
    
    // a. Kosong (Empty)
    cout << "a. Kosong (Empty)      : Top = -1" << endl;
    
    // b. Penuh (Full)
    cout << "b. Penuh (Full)        : Top = n - 1  => Top = 9" << endl;
    
    // c. Bisa diisi (Not Full)
    cout << "c. Bisa diisi (Not Full): Top < n - 1  => Top < 9" << endl;
    
    // d. Ada isinya (Not Empty)
    cout << "d. Ada isinya (Not Empty): Top > -1" << endl;

    return 0;
}