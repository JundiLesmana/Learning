#include <iostream>
using namespace std;

int main() {
    int arr[5] = {10, 20, 30, 40, 50};
    int *ptr = arr; // Nama array 'arr' otomatis merujuk ke alamat elemen pertama

    cout << "=== 1. Ekuivalensi Array dan Pointer ===\n";
    for (int i = 0; i < 5; i++) {
        // arr[i] secara internal diterjemahkan oleh compiler menjadi *(ptr + i)
        cout << "arr[" << i << "] = " << arr[i] 
             << "  <=>  *(ptr+" << i << ") = " << *(ptr + i) << "\n";
    }

    cout << "\n=== 2. Iterasi dengan Pointer Arithmetic ===\n";
    // Tidak pakai indeks 'i', langsung geser pointer (ptr++)
    for (ptr = arr; ptr < arr + 5; ptr++) {
        cout << "Nilai: " << *ptr << " | Alamat: " << ptr << "\n";
    }

    cout << "\n=== 3. Pembuktian Alamat Memori ===\n";
    cout << "Alamat arr     : " << arr << "\n";
    cout << "Alamat &arr[0] : " << &arr[0] << "\n";
    cout << "Apakah sama? " << (arr == &arr[0] ? "YA" : "TIDAK") << "\n";

    return 0;
}