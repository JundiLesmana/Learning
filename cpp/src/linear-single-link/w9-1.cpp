#include <iostream>
#include <limits>
#include <string>

using namespace std;

struct Node {
    int data;
    Node* next;
    explicit Node(int val) : data(val), next(nullptr) {}
};

class SinglyLinkedList {
private:
    Node* head = nullptr;
    Node* tail = nullptr;
    int count = 0;

    static void clearInput() {
        cin.clear();
        cin.ignore(numeric_limits<streamsize>::max(), '\n');
    }

public:
    SinglyLinkedList() = default;
    
    // Mencegah copy yang tidak aman (Best Practice)
    SinglyLinkedList(const SinglyLinkedList&) = delete;
    SinglyLinkedList& operator=(const SinglyLinkedList&) = delete;

    ~SinglyLinkedList() {
        while (head) {
            Node* temp = head;
            head = head->next;
            delete temp;
        }
    }

    static int readInt(const string& prompt) {
        int value;
        while (cout << prompt, !(cin >> value)) {
            cout << "[ERROR] Input tidak valid. Harap masukkan angka.\n";
            clearInput();
        }
        clearInput();
        return value;
    }

    void insertFirst(int val) {
        Node* newNode = new Node(val);
        newNode->next = head;
        head = newNode;
        if (!tail) tail = head;
        count++;
    }

    void insertLast(int val) {
        Node* newNode = new Node(val);
        if (!tail) head = tail = newNode;
        else {
            tail->next = newNode;
            tail = newNode;
        }
        count++;
    }

    void insertAfter(int target, int val) {
        Node* curr = head;
        while (curr && curr->data != target) curr = curr->next;

        if (!curr) {
            cout << "[INFO] Target '" << target << "' tidak ditemukan.\n";
            return;
        }
        Node* newNode = new Node(val);
        newNode->next = curr->next;
        curr->next = newNode;
        if (curr == tail) tail = newNode;
        count++;
    }

    void deleteFirst() {
        if (!head) { cout << "[INFO] Database kosong.\n"; return; }
        Node* temp = head;
        head = head->next;
        if (!head) tail = nullptr;
        delete temp;
        count--;
    }

    void deleteLast() {
        if (!head) { cout << "[INFO] Database kosong.\n"; return; }
        if (head == tail) {
            delete head;
            head = tail = nullptr;
        } else {
            Node* curr = head;
            while (curr->next != tail) curr = curr->next;
            delete tail;
            tail = curr;
            tail->next = nullptr;
        }
        count--;
    }

    void deleteByValue(int val) {
        if (!head) { cout << "[INFO] Database kosong.\n"; return; }
        if (head->data == val) return deleteFirst();

        Node* curr = head;
        while (curr->next && curr->next->data != val) curr = curr->next;

        if (!curr->next) {
            cout << "[INFO] Data '" << val << "' tidak ditemukan.\n";
            return;
        }
        Node* temp = curr->next;
        curr->next = temp->next;
        if (temp == tail) tail = curr;
        delete temp;
        count--;
    }

    void search(int val) const {
        Node* curr = head;
        int pos = 1;
        while (curr) {
            if (curr->data == val) {
                cout << "[FOUND] Data '" << val << "' ditemukan di posisi ke-" << pos << ".\n";
                return;
            }
            curr = curr->next;
            pos++;
        }
        cout << "[NOT FOUND] Data '" << val << "' tidak ada.\n";
    }

    void display() const {
        if (!head) { cout << "[INFO] Database kosong.\n"; return; }
        cout << "\n=== ISI DATABASE (" << count << " data) ===\n";
        cout << "Posisi\t| Data (ID)\n--------+----------\n";
        Node* curr = head;
        for (int pos = 1; curr; curr = curr->next, pos++) {
            cout << pos << "\t| " << curr->data << "\n";
        }
        cout << "================================\n\n";
    }

    int getCount() const { return count; }
};

void displayMenu() {
    cout << "\n========================================\n"
         << "  DATABASE SIMULATOR (Singly Linked List)\n"
         << "========================================\n"
         << "  1. Insert di AWAL    4. Delete di AWAL\n"
         << "  2. Insert di AKHIR   5. Delete di AKHIR\n"
         << "  3. Insert di TENGAH  6. Delete di TENGAH\n"
         << "  7. Tampilkan Data    8. Cari Data\n"
         << "  9. Jumlah Data       0. Keluar\n"
         << "========================================\n";
}

int main() {
    SinglyLinkedList db;
    int choice;

    do {
        displayMenu();
        choice = SinglyLinkedList::readInt("Pilih menu [0-9]: ");
        
        int data, target;
        switch (choice) {
            case 1:
                data = SinglyLinkedList::readInt("Masukkan data (ID) untuk AWAL: ");
                db.insertFirst(data);
                cout << "[SUCCESS] Data ditambahkan di awal.\n";
                break;
            case 2:
                data = SinglyLinkedList::readInt("Masukkan data (ID) untuk AKHIR: ");
                db.insertLast(data);
                cout << "[SUCCESS] Data ditambahkan di akhir.\n";
                break;
            case 3:
                target = SinglyLinkedList::readInt("Masukkan data TARGET: ");
                data = SinglyLinkedList::readInt("Masukkan data BARU: ");
                db.insertAfter(target, data);
                break;
            case 4:
                db.deleteFirst(); // Penanganan kosong sudah ada di dalam method
                break;
            case 5:
                db.deleteLast();
                break;
            case 6:
                data = SinglyLinkedList::readInt("Masukkan data (ID) yang dihapus: ");
                db.deleteByValue(data);
                break;
            case 7:
                db.display();
                break;
            case 8:
                data = SinglyLinkedList::readInt("Masukkan data (ID) yang dicari: ");
                db.search(data);
                break;
            case 9:
                cout << "[INFO] Jumlah data: " << db.getCount() << "\n";
                break;
            case 0:
                cout << "[INFO] Terima kasih telah menggunakan Database Simulator.\n";
                break;
            default:
                cout << "[ERROR] Pilihan tidak valid.\n";
        }
    } while (choice != 0);

    return 0;
}