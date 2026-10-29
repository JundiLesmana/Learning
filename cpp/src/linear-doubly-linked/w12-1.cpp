#include <iostream>
#include <iomanip>
#include <string>
#include <limits>
#include <thread>
#include <chrono>

using namespace std;

struct Mahasiswa {
    string nama;
    string nim;
    char gender;
    double nilai;
};

struct Node {
    Mahasiswa data;
    Node* left;  // Pointer ke simpul sebelumnya
    Node* right; // Pointer ke simpul berikutnya
};

class DoublyLinkedList {
private:
    Node* first;
    Node* last;
    int count;

    // Fungsi bantu untuk animasi loading
    void delayAnimation() {
        this_thread::sleep_for(chrono::milliseconds(400));
    }

    void clearScreen() {
        // Gunakan "cls" jika di Windows, "clear" jika di Linux/Mac
        #ifdef _WIN32
            system("cls");
        #else
            system("clear");
        #endif
    }

public:
    DoublyLinkedList() : first(nullptr), last(nullptr), count(0) {}

    ~DoublyLinkedList() {
        Node* current = first;
        while (current != nullptr) {
            Node* next = current->right;
            delete current;
            current = next;
        }
    }

    bool isEmpty() { return first == nullptr; }

    void insertData(const Mahasiswa& mhs) {
        cout << "\n[ANIMASI INSERT] Memproses data: " << mhs.nama << "...\n";
        
        Node* check = first;
        while (check != nullptr) {
            if (check->data.nim == mhs.nim) {
                cout << ">> GAGAL: NIM " << mhs.nim << " sudah terdaftar!\n";
                return;
            }
            check = check->right;
        }

        Node* newNode = new Node();
        newNode->data = mhs;
        newNode->left = nullptr;
        newNode->right = nullptr;

        if (isEmpty()) {
            cout << "-> List kosong. Menjadikan node ini sebagai FIRST dan LAST.\n";
            first = last = newNode;
        }
        else if (mhs.nim < first->data.nim) {
            cout << "-> NIM lebih kecil dari FIRST. Insert di awal list.\n";
            newNode->right = first;
            first->left = newNode;
            first = newNode;
        }
        else if (mhs.nim > last->data.nim) {
            cout << "-> NIM lebih besar dari LAST. Insert di akhir list.\n";
            last->right = newNode;
            newNode->left = last;
            last = newNode;
        }
        else {
            cout << "-> Mencari posisi di tengah list...\n";
            Node* current = first;
            while (current != nullptr && current->data.nim < mhs.nim) {
                cout << "   Melewati NIM: " << current->data.nim << "...\n";
                delayAnimation();
                current = current->right;
            }
            
            Node* prevNode = current->left;
            
            cout << "   Menyambung pointer di antara NIM " << prevNode->data.nim 
                 << " dan " << current->data.nim << "...\n";
            delayAnimation();

            newNode->right = current;
            newNode->left = prevNode;
            prevNode->right = newNode;
            current->left = newNode;
        }

        count++;
        cout << ">> SUKSES: Data " << mhs.nama << " berhasil ditambahkan.\n";
    }

    void deleteData(const string& targetNim) {
        if (isEmpty()) {
            cout << "\n>> GAGAL: List kosong, tidak ada data untuk dihapus.\n";
            return;
        }

        cout << "\n[ANIMASI HAPUS] Mencari NIM: " << targetNim << "...\n";
        Node* current = first;
        while (current != nullptr && current->data.nim != targetNim) {
            delayAnimation();
            current = current->right;
        }

        if (current == nullptr) {
            cout << ">> GAGAL: NIM " << targetNim << " tidak ditemukan.\n";
            return;
        }

        cout << "-> Data ditemukan: " << current->data.nama << ". Menghapus...\n";
        delayAnimation();

        if (first == last) {
            first = last = nullptr;
        }
        else if (current == first) {
            first = first->right;
            first->left = nullptr;
        }
        else if (current == last) {
            last = last->left;
            last->right = nullptr;
        }
        else {
            Node* prevNode = current->left;
            Node* nextNode = current->right;
            prevNode->right = nextNode;
            nextNode->left = prevNode;
        }

        delete current;
        count--;
        cout << ">> SUKSES: Data dengan NIM " << targetNim << " telah dihapus.\n";
    }

    void printData() {
        cout << "\n[ANIMASI CETAK] Menampilkan Data...\n";
        if (isEmpty()) {
            cout << ">> List saat ini KOSONG.\n";
            return;
        }

        string line(65, '-');
        cout << "\n=== TRAVERSAL MAJU (FIRST -> LAST) ===\n";
        cout << left << setw(15) << "NIM" 
             << setw(20) << "NAMA" 
             << setw(10) << "GENDER" 
             << "NILAI" << "\n" << line << "\n";

        Node* current = first;
        while (current != nullptr) {
            cout << left << setw(15) << current->data.nim
                 << setw(20) << current->data.nama
                 << setw(10) << current->data.gender
                 << fixed << setprecision(1) << current->data.nilai << "\n";
            current = current->right;
            delayAnimation();
        }

        cout << "\n=== TRAVERSAL MUNDUR (LAST -> FIRST) ===\n";
        cout << left << setw(15) << "NIM" 
             << setw(20) << "NAMA" 
             << setw(10) << "GENDER" 
             << "NILAI" << "\n" << line << "\n";

        current = last;
        while (current != nullptr) {
            cout << left << setw(15) << current->data.nim
                 << setw(20) << current->data.nama
                 << setw(10) << current->data.gender
                 << fixed << setprecision(1) << current->data.nilai << "\n";
            current = current->left;
            delayAnimation();
        }
        cout << line << "\nTotal: " << count << " Mahasiswa\n";
    }
};

void clearInputBuffer() {
    cin.clear();
    cin.ignore(numeric_limits<streamsize>::max(), '\n');
}

Mahasiswa inputMahasiswa() {
    Mahasiswa m;
    cout << "Masukkan NAMA   : "; getline(cin, m.nama);
    cout << "Masukkan NIM    : "; getline(cin, m.nim);
    
    while (true) {
        cout << "Masukkan GENDER (L/P): "; cin >> m.gender;
        m.gender = toupper(m.gender);
        if (m.gender == 'L' || m.gender == 'P') break;
        cout << ">> Error: Gender harus 'L' atau 'P'.\n";
    }

    while (true) {
        cout << "Masukkan NILAI  : ";
        if (cin >> m.nilai) break;
        cout << ">> Error: Nilai harus berupa angka.\n";
        clearInputBuffer();
    }
    clearInputBuffer();
    return m;
}

int main() {
    DoublyLinkedList dll;
    int pilihan;

    do {
        cout << "\n==========================================\n";
        cout << "        LINEAR DOUBLY LINKED LIST           \n";
        cout << "==========================================\n";
        cout << "1. INSERT DATA\n";
        cout << "2. HAPUS DATA\n";
        cout << "3. CETAK DATA\n";
        cout << "4. EXIT\n";
        cout << "Pilihan (1 - 4): ";

        if (!(cin >> pilihan)) {
            clearInputBuffer();
            pilihan = 0;
        }

        switch (pilihan) {
            case 1: {
                clearInputBuffer();
                cout << "\n--- INPUT DATA MAHASISWA ---\n";
                Mahasiswa mhs = inputMahasiswa();
                dll.insertData(mhs);
                break;
            }
            case 2: {
                clearInputBuffer();
                if (dll.isEmpty()) {
                    cout << "\n>> List kosong, tidak ada yang bisa dihapus.\n";
                    break;
                }
                string targetNim;
                cout << "\nMasukkan NIM yang akan dihapus: ";
                getline(cin, targetNim);
                dll.deleteData(targetNim);
                break;
            }
            case 3:
                dll.printData();
                break;
            case 4:
                cout << "\nKeluar dari program. Terima kasih!\n";
                break;
            default:
                cout << "\n>> Pilihan tidak valid. Silakan coba lagi.\n";
        }
    } while (pilihan != 4);

    return 0;
}