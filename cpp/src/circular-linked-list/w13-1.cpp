#include <iostream>
#include <string>
using namespace std;

struct Mahasiswa {
    string nama;
    string nim;
    string gender;
    float nilai;
    Mahasiswa* next;
};

class CircularLinkedList {
private:
    Mahasiswa* head;

public:
    CircularLinkedList() : head(nullptr) {}

    void insertData() {
        Mahasiswa* newNode = new Mahasiswa;
        cout << "Nama   : "; cin >> newNode->nama;
        cout << "NIM    : "; cin >> newNode->nim;
        cout << "Gender : "; cin >> newNode->gender;
        cout << "Nilai  : "; cin >> newNode->nilai;

        if (!head) {
            head = newNode;
            newNode->next = head;
            return;
        }

        if (newNode->nim < head->nim) {
            Mahasiswa* tail = head;
            while (tail->next != head) tail = tail->next;
            newNode->next = head;
            head = newNode;
            tail->next = head;
            return;
        }

        Mahasiswa* current = head;
        while (current->next != head && current->next->nim < newNode->nim) {
            current = current->next;
        }
        newNode->next = current->next;
        current->next = newNode;
    }

    void hapusData() {
        if (!head) {
            cout << "List kosong!" << endl;
            return;
        }

        string nim;
        cout << "Masukkan NIM yang akan dihapus: ";
        cin >> nim;

        if (head->nim == nim) {
            if (head->next == head) {
                delete head;
                head = nullptr;
            } else {
                Mahasiswa* tail = head;
                while (tail->next != head) tail = tail->next;
                Mahasiswa* temp = head;
                head = head->next;
                tail->next = head;
                delete temp;
            }
            cout << "Data dengan NIM " << nim << " berhasil dihapus." << endl;
            return;
        }

        Mahasiswa* current = head;
        while (current->next != head && current->next->nim != nim) {
            current = current->next;
        }

        if (current->next->nim == nim) {
            Mahasiswa* temp = current->next;
            current->next = temp->next;
            delete temp;
            cout << "Data dengan NIM " << nim << " berhasil dihapus." << endl;
        } else {
            cout << "Data dengan NIM " << nim << " tidak ditemukan." << endl;
        }
    }

    void cetakData() {
        if (!head) {
            cout << "List kosong!" << endl;
            return;
        }

        Mahasiswa* current = head;
        cout << "\n=== DATA MAHASISWA ===" << endl;
        cout << "NAMA\t\tNIM\t\tGENDER\tNILAI" << endl;
        cout << "----------------------------------------" << endl;
        do {
            cout << current->nama << "\t\t" 
                 << current->nim << "\t\t" 
                 << current->gender << "\t" 
                 << current->nilai << endl;
            current = current->next;
        } while (current != head);
    }
};

int main() {
    CircularLinkedList cll;
    int pilihan;

    do {
        cout << "\nCIRCULAR LINKED LIST" << endl;
        cout << "======================" << endl;
        cout << "1. INSERT DATA" << endl;
        cout << "2. HAPUS DATA" << endl;
        cout << "3. CETAK DATA" << endl;
        cout << "4. EXIT" << endl;
        cout << "Pilihan (1-4): ";
        cin >> pilihan;

        switch (pilihan) {
            case 1: cll.insertData(); break;
            case 2: cll.hapusData(); break;
            case 3: cll.cetakData(); break;
            case 4: cout << "Program selesai." << endl; break;
            default: cout << "Pilihan tidak valid!" << endl;
        }
    } while (pilihan != 4);

    return 0;
}