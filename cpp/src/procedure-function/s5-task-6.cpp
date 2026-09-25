#include <iostream>
#include <iomanip>
using namespace std;

#define JML_RIJ_A 2   
#define JML_KOL_A 3   
#define JML_KOL_B 4   

void mulMatrix(int A[][JML_KOL_A], int B[][JML_KOL_B], int C[][JML_KOL_B]);

int main(){

    int A[JML_RIJ_A][JML_KOL_A];  
    int B[JML_KOL_A][JML_KOL_B];  
    int C[JML_RIJ_A][JML_KOL_B];  

    cout << "===============================================\n";
    cout << "   PERKALIAN MATRIKS (A x B = C)\n";
    cout << "===============================================\n\n";

    cout << "Ukuran matriks A: " << JML_RIJ_A << " x " << JML_KOL_A << "\n"
         << "Ukuran matriks B: " << JML_KOL_A << " x " << JML_KOL_B << "\n"
         << "Ukuran matriks C (hasil): " << JML_RIJ_A << " x " << JML_KOL_B << "\n\n";

    cout << "--- Masukan elemen matriks A (" << JML_RIJ_A << "x" << JML_KOL_A << ") ---\n";
    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_A; j++){
            cout << "  A[" << i << "][" << j << "] = ";
            cin >> A[i][j];
        }
    }

    cout << "\n--- Masukan elemen matriks B (" << JML_KOL_A << "x" << JML_KOL_B << ") ---\n";
    for (int i = 0; i < JML_KOL_A; i++){
        for (int j = 0; j < JML_KOL_B; j++){
            cout << "  B[" << i << "][" << j << "] = ";
            cin >> B[i][j];
        }
    }

    mulMatrix(A, B, C);

    cout << "\n--- Matriks A (" << JML_RIJ_A << "x" << JML_KOL_A << ") ---\n";
    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_A; j++){
            cout << setw(5) << A[i][j];
        }
        cout << endl;
    }

    cout << "\n--- Matriks B (" << JML_KOL_A << "x" << JML_KOL_B << ") ---\n";
    for (int i = 0; i < JML_KOL_A; i++){
        for (int j = 0; j < JML_KOL_B; j++){
            cout << setw(5) << B[i][j];
        }
        cout << endl;
    }

    cout << "\n--- Matriks C = A x B (hasil) (" << JML_RIJ_A << "x" << JML_KOL_B << ") ---\n";
    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_B; j++){
            cout << setw(5) << C[i][j];
        }
        cout << endl;
    }

    cout << "\n===============================================\n";
    cout << "               Selesai. Terima kasih!          \n";
    cout << "===============================================\n\n";

    return 0;
}

void mulMatrix(int A[][JML_KOL_A], int B[][JML_KOL_B], int C[][JML_KOL_B]){

    for (int i = 0; i < JML_RIJ_A; i++){
        for (int j = 0; j < JML_KOL_B; j++){
            C[i][j] = 0;
            for (int k = 0; k < JML_KOL_A; k++){
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }
}