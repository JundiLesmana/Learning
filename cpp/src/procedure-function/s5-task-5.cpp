#include <iostream>
#include <iomanip>
using namespace std;

#define JML_RIJ_A 4
#define JML_KOL_A 3

void transpose(int A[][JML_KOL_A], int B[][JML_RIJ_A]);

int main(){

    int A[JML_RIJ_A][JML_KOL_A];
    int B[JML_KOL_A][JML_RIJ_A];

    cout << "===============================================\n";
    cout << "   TRANSPOSE MATRIKS (A -> B)\n";
    cout << "===============================================\n\n";

    cout << "Ukuran matriks A: " << JML_RIJ_A << " rij x " << JML_KOL_A << " kolom\n"
         << "Ukuran matriks B (hasil): " << JML_KOL_A << " rij x " << JML_RIJ_A << " kolom\n\n";

    cout << "--- Masukan elemen matriks A (4x3) ---\n";
    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_A; j++){
            cout << "  A[" << i << "][" << j << "] = ";
            cin >> A[i][j];
        }
    }

    transpose(A, B);

    cout << "\n--- Matriks A (" << JML_RIJ_A << "x" << JML_KOL_A << ") ---\n";
    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_A; j++){
            cout << setw(5) << A[i][j];
        }
        cout << endl;
    }

    cout << "\n--- Matriks B = transpose(A) (" << JML_KOL_A << "x" << JML_RIJ_A << ") ---\n";
    for (int i = 0; i < JML_KOL_A; i++){
        for (int j = 0; j < JML_RIJ_A; j++){
            cout << setw(5) << B[i][j];
        }
        cout << endl;
    }

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void transpose(int A[][JML_KOL_A], int B[][JML_RIJ_A]){

    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_A; j++){
            B[j][i] = A[i][j];
        }
    }
}