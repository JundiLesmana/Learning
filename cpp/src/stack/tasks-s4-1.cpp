#include <iostream>
using namespace std;

#define MAX 10
int S[MAX];
int top = -1;

int main() {
    int x;
    
    cout << "--- LATIHAN 1: PUSH >= 60, POP < 60 ---" << endl;
    
    while (true) {
        cout << "Input data (999 untuk selesai): ";
        cin >> x;

        // 1. Jika input 999, proses selesai
        if (x == 999) {
            cout << "Proses selesai (Input 999)." << endl;
            break;
        }

        // 2. Jika data >= 60, lakukan PUSH
        if (x >= 60) {
            if (top < MAX - 1) {
                top++;
                S[top] = x;
                cout << "-> " << x << " berhasil di-PUSH." << endl;
            } else { 
                cout << "Stack Penuh" << endl;
                break; 
            }
        } 
        // 3. Jika data < 60, lakukan POP
        else {
            if (top > -1) { 
                cout << "-> Data di-POP: " << S[top] << endl;
                top--;
            } else { 
                cout << "Stack Kosong" << endl;
                break; 
            }
        }
    }
    return 0;
}