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

class QueueLinked {
private:
    Node* head;  
    Node* tail;  
    int   count; 

public:
    QueueLinked() {
        head  = new Node();
        head->next = nullptr;
        tail  = head;
        count = 0;
    }

    ~QueueLinked() {
        clear();
        delete head;
    }

    bool isEmpty() const { return head->next == nullptr; }
    int  size()    const { return count; }

    void enqueue(const Mahasiswa& m) {
        Node* newNode = new Node();
        newNode->data = m;
        newNode->next = nullptr;

        tail->next = newNode;
        tail = newNode;
        ++count;

        animateEnqueue(m);
    }

    bool dequeue(Mahasiswa& out) {
        if (isEmpty()) return false;

        Node* target = head->next;
        out = target->data;

        head->next = target->next;
        if (head->next == nullptr) tail = head; 
        delete target;
        --count;

        animateDequeue(out);
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

        Node* cur = head->next;
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
    }

    void clear() {
        Node* cur = head->next;
        while (cur) {
            Node* nxt = cur->next;
            delete cur;
            cur = nxt;
        }
        head->next = nullptr;
        tail = head;
        count = 0;
    }

private:
    void animateEnqueue(const Mahasiswa& m) const {
        cout << "\n[ENQUEUE] Menambahkan " << m.nama
             << " ke belakang queue...\n";
        for (int i = 0; i < 3; ++i) { cout << "." << flush; /* delay */ }
        cout << " OK\n";
    }
    void animateDequeue(const Mahasiswa& m) const {
        cout << "\n[DEQUEUE] Mengeluarkan " << m.nama
             << " dari depan queue...\n";
        for (int i = 0; i < 3; ++i) { cout << "." << flush; }
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
    QueueLinked q;
    int pilihan;

    do {
        cout << "\n=========================================\n";
        cout << "   QUEUE MAHASISWA (LINKED LIST + HEAD)\n";
        cout << "=========================================\n";
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