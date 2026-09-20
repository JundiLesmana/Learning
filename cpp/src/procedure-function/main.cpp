/* =====================================================================
   File    : src/procedure-function/main.cpp
   Tujuan  : Latihan membuat fungsi (procedure-function) dalam C++
   Isi     :
       Soal 1 : fungsi abs, minus, pangkatDua  (1 parameter masukan)
       Soal 2 : fungsi max, min, odd, even     (2 parameter masukan)
       Soal 3 : fungsi luas, keliling, diameter (lingkaran, 1 parameter)
   ===================================================================== */

#include <iostream>

using namespace std;

/* Konstanta PI untuk perhitungan lingkaran pada Soal 3 */
const double PI = 3.141592653589793;

/* ---------------- Prototipe fungsi Soal 1 (1 parameter) ---------------- */
int abs(int x);          // mengembalikan nilai positif dari suatu angka
int minus(int x);        // positif -> negatif, negatif -> positif
int pangkatDua(int x);   // mengembalikan nilai angka dipangkatkan dua

/* ---------------- Prototipe fungsi Soal 2 (2 parameter) ---------------- */
int max(int a, int b);   // mengembalikan maksimum dari 2 bilangan
int min(int a, int b);   // mengembalikan minimum dari 2 bilangan
int odd(int a, int b);   // mengembalikan bilangan yang ganjil
int even(int a, int b);  // mengembalikan bilangan yang genap

/* ---------------- Prototipe fungsi Soal 3 (lingkaran) ---------------- */
double luas(double r);       // menghitung luas lingkaran
double keliling(double r);   // menghitung keliling lingkaran
double diameter(double r);   // menghitung diameter lingkaran

/* =====================================================================
   Program utama: menjalankan Soal 1, Soal 2, dan Soal 3 berurutan
   ===================================================================== */
int main(){

    cout << "===============================================\n";
    cout << "   PROGRAM LATIHAN FUNGSI (PROCEDURE-FUNCTION)\n";
    cout << "===============================================\n\n";

    /* Catatan:
       Awalan "::" pada pemanggilan fungsi (mis. ::abs, ::minus, ::max, ::min)
       digunakan agar yang dipanggil adalah fungsi milik kita sendiri, bukan
       nama serupa yang ada di namespace std (mis. std::minus, std::max)
       sehingga kompilasi tidak menjadi ambigu. */

    /* ------------------------------- SOAL 1 ------------------------------- */
    cout << "-------------------- SOAL 1 --------------------\n";
    cout << "     abs, minus, pangkat dua (1 parameter)\n";
    cout << "------------------------------------------------\n";

    int angka1;  // angka masukan untuk Soal 1

    cout << "Masukkan satu angka: ";
    cin >> angka1;

    cout << "  Nilai absolut  dari " << angka1 << " = " << ::abs(angka1)        << "\n";
    cout << "  Nilai minus    dari " << angka1 << " = " << ::minus(angka1)      << "\n";
    cout << "  Nilai pangkat 2 dari " << angka1 << " = " << ::pangkatDua(angka1) << "\n\n";

    /* ------------------------------- SOAL 2 ------------------------------- */
    cout << "-------------------- SOAL 2 --------------------\n";
    cout << "   max, min, odd, even (2 parameter)\n";
    cout << "------------------------------------------------\n";

    int a, b;  // dua angka masukan untuk Soal 2

    cout << "Masukkan dua angka: ";
    cin >> a >> b;

    cout << "  max("  << a << ", " << b << ")  = " << ::max(a, b)  << "\n";
    cout << "  min("  << a << ", " << b << ")  = " << ::min(a, b)  << "\n";
    cout << "  odd("  << a << ", " << b << ")  = " << odd(a, b)    << "\n";
    cout << "  even(" << a << ", " << b << ")  = " << even(a, b)   << "\n\n";

    /* ------------------------------- SOAL 3 ------------------------------- */
    cout << "-------------------- SOAL 3 --------------------\n";
    cout << "   luas, keliling, diameter lingkaran\n";
    cout << "------------------------------------------------\n";

    double r;  // jari-jari lingkaran untuk Soal 3

    cout << "Masukkan jari-jari lingkaran: ";
    cin >> r;

    cout << "  Luas lingkaran     = " << luas(r)     << "\n";
    cout << "  Keliling lingkaran = " << keliling(r) << "\n";
    cout << "  Diameter lingkaran = " << diameter(r) << "\n\n";

    cout << "===============================================\n";
    cout << "               Selesai. Terima kasih!\n";
    cout << "===============================================\n\n";

    return 0;
}
/* =====================================================================
   Definisi fungsi Soal 1
   ===================================================================== */

/* Mengembalikan nilai positif dari suatu angka (nilai absolut). */
int abs(int x){
    if (x < 0){        // bila angka negatif
        return -x;     // kembalikan nilai positifnya
    }
    return x;          // angka positif -> kembalikan apa adanya
}

/* Mengubah tanda bilangan: positif -> negatif, negatif -> positif. */
int minus(int x){
    return -x;
}

/* Mengembalikan nilai angka yang dipangkatkan dua (kuadrat). */
int pangkatDua(int x){
    return x * x;
}

/* =====================================================================
   Definisi fungsi Soal 2
   ===================================================================== */

/* Mengembalikan nilai maksimum (terbesar) dari 2 bilangan. */
int max(int a, int b){
    if (a > b){
        return a;
    }
    return b;
}

/* Mengembalikan nilai minimum (terkecil) dari 2 bilangan. */
int min(int a, int b){
    if (a < b){
        return a;
    }
    return b;
}

/* Mengembalikan bilangan yang ganjil.
   Bila keduanya ganjil, boleh dipilih salah satu (di sini dipilih a). */
int odd(int a, int b){
    if (a % 2 != 0){     // a ganjil -> kembalikan a
        return a;
    }
    if (b % 2 != 0){     // a genap dan b ganjil -> kembalikan b
        return b;
    }
    return a;            // keduanya genap (tidak ada yang ganjil)
}

/* Mengembalikan bilangan yang genap.
   Bila keduanya genap, boleh dipilih salah satu (di sini dipilih a). */
int even(int a, int b){
    if (a % 2 == 0){     // a genap -> kembalikan a
        return a;
    }
    if (b % 2 == 0){     // a ganjil dan b genap -> kembalikan b
        return b;
    }
    return b;            // keduanya ganjil (tidak ada yang genap)
}

/* =====================================================================
   Definisi fungsi Soal 3
   ===================================================================== */

/* Menghitung luas lingkaran: luas = PI x r^2 */
double luas(double r){
    return PI * r * r;
}

/* Menghitung keliling lingkaran: keliling = 2 x PI x r */
double keliling(double r){
    return 2 * PI * r;
}

/* Menghitung diameter lingkaran: diameter = 2 x r */
double diameter(double r){
    return 2 * r;
}
