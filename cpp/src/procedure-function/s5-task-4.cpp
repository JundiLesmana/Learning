#include <iostream>
#include <iomanip>
using namespace std;

#define JML_RIJ 3 
#define JML_KOL 3 

void sumMatrix(int A[][JML_KOL], int B[][JML_KOL], int C[][JML_KOL]);

int main(){

    int A[JML_RIJ][JML_KOL];
    int B[JML_RIJ][JML_KOL];
    int C[JML_RIJ][JML_KOL];

    cout << "===============================================\n";
    cout << "   PERJUMLAHAN MATRIKS (A + B = C)\n";
    cout << "===============================================\n\n";

    cout << "--- Masukan elemen matriks A (" << JML_RIJ << "x" << JML_KOL << ") ---\n";
    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            cout << "  A[" << i << "][" << j << "] = ";
            cin >> A[i][j];
        }
    }

    cout << "\n--- Masukan elemen matriks B (" << JML_RIJ << "x" << JML_KOL << ") ---\n";
    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            cout << "  B[" << i << "][" << j << "] = ";
            cin >> B[i][j];
        }
    }

    sumMatrix(A, B, C);

    cout << "\n--- Matriks A ---\n";
    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            cout << setw(5) << A[i][j];
        }
        cout << endl;
    }

    cout << "\n--- Matriks B ---\n";
    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            cout << setw(5) << B[i][j];
        }
        cout << endl;
    }

    cout << "\n--- Matriks C = A + B (hasil) ---\n";
    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            cout << setw(5) << C[i][j];
        }
        cout << endl;
    }

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void sumMatrix(int A[][JML_KOL], int B[][JML_KOL], int C[][JML_KOL]){

    for (int i = 0; i < JML_RIJ; i++){
        for (int j = 0; j < JML_KOL; j++){
            C[i][j] = A[i][j] + B[i][j];
        }
    }
}