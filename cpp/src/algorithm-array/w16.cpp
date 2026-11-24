#include <iostream>
#include <iomanip>
#include <limits>

using namespace std;

constexpr int N = 5;
constexpr double PASS_GRADE = 75.0;

int bacaNilai(int indeks) {
    int nilai;
    while (true) {
        cout << "  Nilai ke-" << indeks + 1 << " (0-100): ";
        if (cin >> nilai && nilai >= 0 && nilai <= 100) return nilai;
        cin.clear();
        cin.ignore(numeric_limits<streamsize>::max(), '\n');
        cout << "  [!] Input tidak valid. Masukkan angka 0-100.\n";
    }
}

int main() {
    int nilai[N]{};
    int total = 0;
    int maks = numeric_limits<int>::min();
    int mins = numeric_limits<int>::max();

    cout << "=== PROGRAM ARRAY: STATISTIK NILAI ===\n";
    cout << "Masukkan " << N << " nilai mahasiswa:\n";

    for (int i = 0; i < N; ++i) {
        nilai[i] = bacaNilai(i);
        total += nilai[i];
        if (nilai[i] > maks) maks = nilai[i];
        if (nilai[i] < mins) mins = nilai[i];
    }

    const double rata = static_cast<double>(total) / N;

    cout << "\n-------------------------------------\n";
    cout << "Data nilai : ";
    for (int i = 0; i < N; ++i) {
        cout << nilai[i] << (i < N - 1 ? ", " : "\n");
    }

    cout << fixed << setprecision(2);
    cout << "Total      : " << total << '\n'
         << "Rata-rata  : " << rata << '\n'
         << "Tertinggi  : " << maks << '\n'
         << "Terendah   : " << mins << '\n'
         << "Status     : " << (rata >= PASS_GRADE ? "LULUS" : "TIDAK LULUS") << '\n';
    cout << "-------------------------------------\n";

    return 0;
}