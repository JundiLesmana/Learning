#include <iostream>
#include <string>
using namespace std;

struct Mahasiswa {
    string nama;
    string nim;
    string gender;
    float nilai;
};

struct Node {
    Mahasiswa data;
    Node* left;
    Node* right;
};

Node* FIRST = NULL;
Node* LAST = NULL;

Node* buatSimpul(string nama, string nim, string gender, float nilai) {
    Node* p = new Node;
    p->data.nama = nama;
    p->data.nim = nim;
    p->data.gender = gender;
    p->data.nilai = nilai;
    p->left = NULL;
    p->right = NULL;
    return p;
}

void insertSorted(Node* p) {
    if (FIRST == NULL) {
        FIRST = p;
        LAST = p;
        p->right = p;
        p->left = p;
        return;
    }

    if (p->data.nim < FIRST->data.nim) {
        p->right = FIRST;
        p->left = LAST;
        FIRST->left = p;
        LAST->right = p;
        FIRST = p;
        return;
    }

    if (p->data.nim > LAST->data.nim) {
        p->left = LAST;
        p->right = FIRST;
        LAST->right = p;
        FIRST->left = p;
        LAST = p;
        return;
    }

    Node* curr = FIRST;
    while (curr->right != FIRST && curr->right->data.nim < p->data.nim) {
        curr = curr->right;
    }

    p->right = curr->right;
    p->left = curr;
    curr->right->left = p;
    curr->right = p;
}

void insertData() {
    string nama, nim, gender;
    float nilai;

    cout << "\n--- INSERT DATA ---\n";
    cout << "Nama   : "; getline(cin, nama);
    cout << "NIM    : "; getline(cin, nim);
    cout << "Gender : "; getline(cin, gender);
    cout << "Nilai  : "; cin >> nilai;
    cin.ignore();

    Node* p = buatSimpul(nama, nim, gender, nilai);
    insertSorted(p);

    cout << "Data berhasil ditambahkan.\n";
}

void hapusData() {
    if (FIRST == NULL) {
        cout << "\nList kosong.\n";
        return;
    }

    string nim;
    cout << "\n--- HAPUS DATA ---\n";
    cout << "Masukkan NIM yang akan dihapus: ";
    getline(cin, nim);

    Node* curr = FIRST;
    Node* prev = NULL;
    bool found = false;

    do {
        if (curr->data.nim == nim) {
            found = true;
            break;
        }
        prev = curr;
        curr = curr->right;
    } while (curr != FIRST);

    if (!found) {
        cout << "Data dengan NIM " << nim << " tidak ditemukan.\n";
        return;
    }

    if (FIRST == LAST) {
        delete FIRST;
        FIRST = NULL;
        LAST = NULL;
    } else if (curr == FIRST) {
        FIRST = FIRST->right;
        LAST->right = FIRST;
        FIRST->left = LAST;
        delete curr;
    } else if (curr == LAST) {
        LAST = LAST->left;
        LAST->right = FIRST;
        FIRST->left = LAST;
        delete curr;
    } else {
        prev->right = curr->right;
        curr->right->left = prev;
        delete curr;
    }

    cout << "Data dengan NIM " << nim << " berhasil dihapus.\n";
}

void cetakData() {
    if (FIRST == NULL) {
        cout << "\nList kosong.\n";
        return;
    }

    cout << "\n--- CETAK DATA ---\n";
    cout << "================================================================\n";
    cout << "NIM\t\tNama\t\tGender\tNilai\n";
    cout << "================================================================\n";

    Node* curr = FIRST;
    do {
        cout << curr->data.nim << "\t"
             << curr->data.nama << "\t"
             << curr->data.gender << "\t"
             << curr->data.nilai << "\n";
        curr = curr->right;
    } while (curr != FIRST);

    cout << "================================================================\n";
}

int main() {
    int pilihan;

    do {
        cout << "\n      CIRCULAR    DOUBLY    LINKED    LIST\n";
        cout << "==========================================\n";
        cout << "1. INSERT DATA\n";
        cout << "2. HAPUS DATA\n";
        cout << "3. CETAK DATA\n";
        cout << "4. EXIT\n";
        cout << "Pilihan (1-4): ";
        cin >> pilihan;
        cin.ignore();

        switch (pilihan) {
            case 1: insertData(); break;
            case 2: hapusData(); break;
            case 3: cetakData(); break;
            case 4: cout << "Program selesai.\n"; break;
            default: cout << "Pilihan tidak valid.\n";
        }
    } while (pilihan != 4);

    return 0;
}