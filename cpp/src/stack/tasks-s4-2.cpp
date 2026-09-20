#include <iostream>
using namespace std;

#define MAX 10
int S[MAX];
int top = -1;

int main() {
    int x;
    
    cout << "--- LATIHAN 2: PUSH sampai Penuh/999, lalu POP semua ---" << endl;
    
    // Fase 1: Input dan PUSH
    cout << "[Fase PUSH] Input data (999 untuk stop):" << endl;
    while (true) {
        cout << "Input data: ";
        cin >> x;
        
        if (x == 999) {
            break; 
        }
        
        top++;
        S[top] = x;
        cout << "-> " << x << " di-PUSH." << endl;
        
        if (top == MAX - 1) {
            cout << "(Stack telah penuh, input dihentikan)" << endl;
            break; 
        }
    }

    // Fase 2: POP sampai Kosong
    cout << "\n[Fase POP] Mengeluarkan semua isi stack:" << endl;
    while (top > -1) { 
        cout << S[top] << endl;
        top--;
    }
    cout << "Stack telah kosong." << endl;

    return 0;
}