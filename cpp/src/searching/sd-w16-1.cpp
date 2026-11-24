#include <iostream>
#include <vector>
#include <algorithm>
#include <limits>

using namespace std;

void tampilkanArray(const vector<int>& arr) {
    if (arr.empty()) {
        cout << "Array kosong" << endl;
        return;
    }
    for (int nilai : arr) {
        cout << nilai << " ";
    }
    cout << endl;
}

int cariLinier(const vector<int>& arr, int target) {
    for (size_t i = 0; i < arr.size(); ++i) {
        if (arr[i] == target) {
            return static_cast<int>(i);
        }
    }
    return -1;
}

int cariBiner(const vector<int>& arr, int target) {
    if (arr.empty()) return -1;
    
    int kiri = 0;
    int kanan = static_cast<int>(arr.size()) - 1;
    
    while (kiri <= kanan) {
        int tengah = kiri + (kanan - kiri) / 2;
        
        if (arr[tengah] == target) {
            return tengah;
        }
        if (arr[tengah] < target) {
            kiri = tengah + 1;
        } else {
            kanan = tengah - 1;
        }
    }
    return -1;
}

int bacaAngkaValid(const string& pesan) {
    int nilai;
    while (true) {
        cout << pesan;
        if (cin >> nilai) {
            return nilai;
        }
        cout << "Input tidak valid. Masukkan angka." << endl;
        cin.clear();
        cin.ignore(numeric_limits<streamsize>::max(), '\n');
    }
}

vector<int> ambilInputArray() {
    int jumlah = bacaAngkaValid("Masukkan jumlah elemen: ");
    
    while (jumlah <= 0) {
        cout << "Jumlah harus lebih dari 0." << endl;
        jumlah = bacaAngkaValid("Masukkan jumlah elemen: ");
    }
    
    vector<int> arr;
    arr.reserve(jumlah);
    
    cout << "Masukkan " << jumlah << " elemen array :" << endl;
    for (int i = 0; i < jumlah; ++i) {
        int nilai = bacaAngkaValid("Elemen ke-" + to_string(i + 1) + ": ");
        arr.push_back(nilai);
    }
    return arr;
}

void eksekusiCariLinier() {
    cout << "\n=== Linear Search ===" << endl;
    vector<int> arr = ambilInputArray();
    
    cout << "\nArray yang Anda masukkan: ";
    tampilkanArray(arr);
    
    int target = bacaAngkaValid("Masukkan nilai yang dicari: ");
    int indeks = cariLinier(arr, target);
    
    if (indeks != -1) {
        cout << "Ditemukan di indeks: " << indeks << endl;
    } else {
        cout << "Nilai " << target << " tidak ditemukan." << endl;
    }
}

void eksekusiCariBiner() {
    cout << "\n=== Binary Search ===" << endl;
    vector<int> arr = ambilInputArray();
    
    sort(arr.begin(), arr.end());
    cout << "\nArray setelah diurutkan: ";
    tampilkanArray(arr);
    
    int target = bacaAngkaValid("Masukkan nilai yang dicari: ");
    int indeks = cariBiner(arr, target);
    
    if (indeks != -1) {
        cout << "Ditemukan di indeks: " << indeks << endl;
    } else {
        cout << "Nilai " << target << " tidak ditemukan." << endl;
    }
}

int main() {
    int pilihan;
    do {
        cout << "\n=== Menu Pencarian ===" << endl;
        cout << "1. Linear Search" << endl;
        cout << "2. Binary Search" << endl;
        cout << "3. Keluar" << endl;
        pilihan = bacaAngkaValid("Pilih menu (1-3): ");

        switch (pilihan) {
            case 1:
                eksekusiCariLinier();
                break;
            case 2:
                eksekusiCariBiner();
                break;
            case 3:
                cout << "Keluar dari program." << endl;
                break;
            default:
                cout << "Pilihan tidak valid. Masukkan 1, 2, atau 3." << endl;
        }
    } while (pilihan != 3);

    return 0;
}