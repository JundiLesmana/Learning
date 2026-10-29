#include <iostream>
#include <iomanip>
#include <string>
#include <limits>
using namespace std;

struct Mahasiswa {
    string nama;
    string nim;
    char   gender;
    double nilai;
};

struct Node {
    Mahasiswa data;
    Node* next;
};

class QueueNoHead {
private:
    Node* front; 
    Node* rear;  
    int   count;

public:
    QueueNoHead() : front(nullptr), rear(nullptr), count(0) {}

    ~QueueNoHead() { clear(); }

    bool isEmpty() const { return front == nullptr; }
    int  size()    const { return count; }

    void enqueue(const Mahasiswa& m) {
        Node* newNode = new Node();
        newNode->data = m;
        newNode->next = nullptr;

        if (isEmpty()) {
            front = rear = newNode;  
        } else {
            rear->next = newNode;
            rear = newNode;
        }
        ++count;

        animate("ENQUEUE", m.nama, "ke belakang queue");
    }

    bool dequeue(Mahasiswa& out) {
        if (isEmpty()) return false;

        Node* target = front;
        out = target->data;

        front = front->next;

        if (front == nullptr) {
            rear = nullptr; 
        }
        delete target;
        --count;

        animate("DEQUEUE", out.nama, "dari depan queue");
        return true;
    }

    void display() const {
        cout << "\n=== ISI QUEUE (FRONT -> REAR) ===\n";
        if (isEmpty()) {
            cout << "[ Queue kosong ]\n";
            return;
        }

        cout << left
             << setw(4)  << "No"
             << setw(20) << "NAMA"
             << setw(15) << "NIM"
             << setw(8)  << "GENDER"
             << setw(8)  << "NILAI" << "\n";
        cout << string(55, '-') << "\n";

        Node* cur = front;
        int i = 1;
        while (cur) {
            cout << left
                 << setw(4)  << i++
                 << setw(20) << cur->data.nama
                 << setw(15) << cur->data.nim
                 << setw(8)  << cur->data.gender
                 << setw(8)  << fixed << setprecision(2) << cur->data.nilai
                 << "\n";
            cur = cur->next;
        }
        cout << "Total: " << count << " mahasiswa\n";
        cout << "FRONT = " << (front ? front->data.nama : "-") << " | ";
        cout << "REAR  = " << (rear  ? rear->data.nama  : "-") << "\n";
    }

    void clear() {
        Node* cur = front;
        while (cur) {
            Node* nxt = cur->next;
            delete cur;
            cur = nxt;
        }
        front = rear = nullptr;
        count = 0;
    }

private:
    static void animate(const string& aksi,
                        const string& nama,
                        const string& ket) {
        cout << "\n[" << aksi << "] " << nama << " " << ket << "...\n";
        for (int i = 0; i < 3; ++i) cout << "." << flush;
        cout << " OK\n";
    }
};

void clearInput() {
    cin.clear();
    cin.ignore(numeric_limits<streamsize>::max(), '\n');
}

Mahasiswa inputMahasiswa() {
    Mahasiswa m;
    cout << "Nama   : "; getline(cin, m.nama);
    cout << "NIM    : "; getline(cin, m.nim);
    do {
        cout << "Gender (L/P): "; cin >> m.gender;
        m.gender = toupper(m.gender);
        if (m.gender != 'L' && m.gender != 'P')
            cout << "  ! Gender harus L atau P.\n";
    } while (m.gender != 'L' && m.gender != 'P');
    cout << "Nilai  : ";
    while (!(cin >> m.nilai)) {
        cout << "  ! Nilai harus angka: ";
        clearInput();
    }
    clearInput();
    return m;
}

int main() {
    QueueNoHead q;
    int pilihan;

    do {
        cout << "\n============================================\n";
        cout << "  LINKED LIST TANPA HEAD\n";
        cout << "============================================\n";
        cout << "1. INSERT DATA (Enqueue)\n";
        cout << "2. HAPUS DATA  (Dequeue)\n";
        cout << "3. CETAK DATA\n";
        cout << "4. EXIT\n";
        cout << "Pilihan (1-4): ";

        if (!(cin >> pilihan)) { clearInput(); pilihan = 0; }

        switch (pilihan) {
        case 1: {
            clearInput();
            cout << "\n--- INPUT DATA MAHASISWA ---\n";
            Mahasiswa m = inputMahasiswa();
            q.enqueue(m);
            break;
        }
        case 2: {
            Mahasiswa m;
            if (q.dequeue(m))
                cout << "Data " << m.nama << " berhasil dihapus.\n";
            else
                cout << "! Queue kosong, tidak ada data untuk dihapus.\n";
            break;
        }
        case 3:
            q.display();
            break;
        case 4:
            cout << "Keluar dari program. Terima kasih.\n";
            break;
        default:
            cout << "! Pilihan tidak valid.\n";
        }
    } while (pilihan != 4);

    return 0;
}