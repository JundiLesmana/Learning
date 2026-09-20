#include <iostream>
using namespace std;

#define MAX 10
int S[MAX];
int top = MAX - 1; 

int main() {
    cout << "--- LATIHAN 4: POP sampai Stack KOSONG ---" << endl;
    
    // Loop akan terus berjalan selama kondisi ADA ISINYA (top > -1)
    while (top > -1) {
        cout << "Data di-POP: " << S[top] << endl;
        top--;
    }
    
    cout << "Proses selesai. Stack telah KOSONG (Top = " << top << ")." << endl;
    
    return 0;
}