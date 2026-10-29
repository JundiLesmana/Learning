#include <iostream>

using namespace std;

// 1. STRUKTUR NODE
struct Node {
    int data;
    Node* next;
    Node(int val, Node* nxt = NULL) : data(val), next(nxt) {}
};

// 2. KLAS STACK
class LLStack {
    Node* top;

public:
    LLStack() : top(NULL) {}
    
    // Destructor otomatis membersihkan memori (Mencegah Memory Leak)
    ~LLStack() {
        while (top != NULL) {
            Node* temp = top;
            top = top->next;
            delete temp;
        }
    }

    bool isEmpty() const { return top == NULL; }

    // PUSH: Tambah data di puncak (O(1))
    void push(int val) {
        top = new Node(val, top); // Buat node baru, tunjuk ke top lama, update top
    }

    // POP: Hapus data di puncak (O(1))
    void pop() {
        if (isEmpty()) { cout << "[EMPTY] Stack kosong!\n"; return; }
        Node* temp = top;
        top = top->next;
        delete temp;
    }

    // PEEK: Lihat data di puncak tanpa menghapus
    int peek() const {
        if (isEmpty()) { cout << "[EMPTY] Stack kosong!\n"; return -1; }
        return top->data;
    }

    // DISPLAY: Tampilkan isi stack
    void display() const {
        cout << "Stack (Top -> Bottom): ";
        for (Node* c = top; c != NULL; c = c->next) {
            cout << c->data << " -> ";
        }
        cout << "NULL\n";
    }
};

int main() {
    LLStack s;

    s.push(10);
    s.push(20);
    s.push(30);
    s.display();        // Stack: 30 -> 20 -> 10 -> NULL

    cout << "Puncak: " << s.peek() << "\n"; // Output: 30

    s.pop();            // Hapus 30
    s.pop();            // Hapus 20
    s.display();        // Stack: 10 -> NULL

    s.pop();
    s.pop();            // [EMPTY] Stack kosong!

    return 0; // Destructor otomatis menghapus node '10' yang tersisa
}