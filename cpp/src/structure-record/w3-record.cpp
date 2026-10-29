#include <iostream>
#include <string>
#include <iomanip> 

using namespace std;

struct Mahasiswa {
    string nim;
    string nama;
    float ipk;
};

void tampilkanData(const Mahasiswa& m) {
    cout << "NIM  : " << m.nim << endl;
    cout << "Nama : " << m.nama << endl;
    cout << "IPK  : " << fixed << setprecision(2) << m.ipk << endl;
    cout << "------------------------" << endl;
}

Mahasiswa buatData(string n, string nm, float i) {
    Mahasiswa m;
    m.nim = n;
    m.nama = nm;
    m.ipk = i;
    return m;
}

int main() {
    Mahasiswa mhs1;
    mhs1.nim = "2023001";
    mhs1.nama = "Budi Santoso";
    mhs1.ipk = 3.75;

    cout << "=== Data Mahasiswa 1 ===" << endl;
    tampilkanData(mhs1);


    const int JUMLAH_MHS = 3;
    Mahasiswa daftarMhs[JUMLAH_MHS];
    daftarMhs[0] = buatData("2023002", "Andi Pratama", 3.50);
    daftarMhs[1] = buatData("2023003", "Siti Aminah", 3.85);
    daftarMhs[2] = buatData("2023004", "Joko Widodo", 2.90);

    cout << "\n=== Daftar Semua Mahasiswa ===" << endl;
    for (int i = 0; i < JUMLAH_MHS; i++) {
        cout << "Data ke-" << (i + 1) << ":" << endl;
        tampilkanData(daftarMhs[i]);
    }

    daftarMhs[2].ipk = 3.10; 
    
    cout << "\n=== Update IPK ===" << endl;
    cout << "IPK Baru " << daftarMhs[2].nama << ": ";
    cout << fixed << setprecision(2) << daftarMhs[2].ipk << endl;

    return 0;
}