#include <iostream>

using namespace std;

struct Node {
    int data;
    Node* next;
    
    Node(int val, Node* nxt = NULL) : data(val), next(nxt) {}
};

class LinkedList {
private:
    Node* head;

public:
    // Constructor eksplisit
    LinkedList() : head(NULL) {}

    ~LinkedList() {
        clear();
    }
    void push_front(int val) {
        head = new Node(val, head);
    }
    void push_back(int val) {
        if (head == NULL) {
            head = new Node(val);
            return;
        }
        Node* curr = head;
        while (curr->next != NULL) {
            curr = curr->next;
        }
        curr->next = new Node(val);
    }

    void pop_front() {
        if (head == NULL) return;
        Node* temp = head;
        head = head->next;
        delete temp;
    }

    void clear() {
        while (head != NULL) {
            pop_front();
        }
    }

    void print() const {
        Node* curr = head;
        while (curr != NULL) {
            cout << curr->data << " -> ";
            curr = curr->next;
        }
        cout << "NULL\n";
    }
};

int main() {
    LinkedList list;

    list.push_back(10);
    list.push_back(20);
    list.push_front(5);
    
    cout << "Isi List: ";
    list.print(); 

    list.pop_front();
    cout << "Setelah pop_front: ";
    list.print(); 

    return 0;
}