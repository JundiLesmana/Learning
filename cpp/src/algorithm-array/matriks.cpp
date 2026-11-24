#include <iostream>
#include <iomanip>
#include <limits>
#include <string>

using namespace std;

constexpr int ROWS = 3;
constexpr int COLS = 3;

using Matriks = int[ROWS][COLS];

int bacaAngka(const string& prompt) {
    int x;
    while (true) {
        cout << prompt;
        if (cin >> x) return x;
        cin.clear();
        cin.ignore(numeric_limits<streamsize>::max(), '\n');
        cout << "  [!] Masukkan bilangan bulat yang valid.\n";
    }
}

void tampilMatriks(const Matriks m, const string& judul) {
    cout << "\n" << judul << "\n";
    cout << "  +---------------------+\n";
    for (int i = 0; i < ROWS; ++i) {
        cout << "  |";
        for (int j = 0; j < COLS; ++j) {
            cout << setw(6) << m[i][j] << ' ';
        }
        cout << "|\n";
    }
    cout << "  +---------------------+\n";
}

void transpose(const Matriks src, Matriks dst) {
    for (int i = 0; i < ROWS; ++i)
        for (int j = 0; j < COLS; ++j)
            dst[j][i] = src[i][j];
}

long long determinan3x3(const Matriks m) {
    return static_cast<long long>(m[0][0]) * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
         - static_cast<long long>(m[0][1]) * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
         + static_cast<long long>(m[0][2]) * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
}

void statistik(const Matriks m) {
    long long total = 0;
    int diagUtama = 0, diagSekunder = 0;

    cout << "\n--- ANALISIS MATRIKS ---\n";
    for (int i = 0; i < ROWS; ++i) {
        long long sumBaris = 0;
        for (int j = 0; j < COLS; ++j) {
            sumBaris += m[i][j];
            total += m[i][j];
        }
        cout << "Jumlah baris " << i + 1 << "   : " << sumBaris << '\n';
        diagUtama += m[i][i];
        diagSekunder += m[i][COLS - 1 - i];
    }

    for (int j = 0; j < COLS; ++j) {
        long long sumKolom = 0;
        for (int i = 0; i < ROWS; ++i) sumKolom += m[i][j];
        cout << "Jumlah kolom " << j + 1 << "   : " << sumKolom << '\n';
    }

    cout << "Total elemen     : " << total << '\n'
         << "Diagonal utama   : " << diagUtama << '\n'
         << "Diagonal sekunder: " << diagSekunder << '\n'
         << "Determinan       : " << determinan3x3(m) << '\n';
}

int main() {
    Matriks m{};
    Matriks mt{};

    cout << "=====================================\n";
    cout << "   PROGRAM MATRIKS 3x3 (ARRAY 2D)    \n";
    cout << "=====================================\n";
    cout << "Masukkan 9 elemen matriks:\n";

    for (int i = 0; i < ROWS; ++i) {
        for (int j = 0; j < COLS; ++j) {
            m[i][j] = bacaAngka("  M[" + to_string(i) + "][" + to_string(j) + "] = ");
        }
    }

    tampilMatriks(m, "MATRIKS ASLI (3x3):");
    transpose(m, mt);
    tampilMatriks(mt, "MATRIKS TRANSPOSE:");
    statistik(m);

    cout << "\n--- PEMETAAN INDEKS ---\n";
    for (int i = 0; i < ROWS; ++i)
        for (int j = 0; j < COLS; ++j)
            cout << "  m[" << i << "][" << j << "] = " << setw(4) << m[i][j] << '\n';

    return 0;
}