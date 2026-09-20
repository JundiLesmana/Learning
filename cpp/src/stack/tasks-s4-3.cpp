#include <iostream>
using namespace std;

#define MAX 10
int S[MAX];
int top = -1;

int main() {
    int x;
    
    cout << "--- LATIHAN 3: PUSH sampai Stack PENUH ---" << endl;
    
    // Loop akan terus berjalan selama kondisi BISA DIISI (top < n-1)
    while (top < MAX - 1) {
        cout << "Input data ke-" << (top + 2) << ": ";
        cin >> x;
        
        top++;
        S[top] = x;
    }
    
    cout << "Proses selesai. Stack telah PENUH (Top = " << top << ")." << endl;
    
    return 0;
}