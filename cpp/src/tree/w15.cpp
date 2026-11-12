#include <iostream>
#include <queue>
using namespace std;

struct Node {
    int data;
    Node* left;
    Node* right;
};

Node* createNode(int data) {
    Node* newNode = new Node();
    newNode->data = data;
    newNode->left = nullptr;
    newNode->right = nullptr;
    return newNode;
}

void insertNode(Node*& root, int data) {
    if (root == nullptr) {
        root = createNode(data);
        return;
    }
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
        Node* temp = q.front();
        q.pop();
        if (temp->left == nullptr) {
            temp->left = createNode(data);
            break;
        } else {
            q.push(temp->left);
        }
        if (temp->right == nullptr) {
            temp->right = createNode(data);
            break;
        } else {
            q.push(temp->right);
        }
    }
}

void deleteDeepestNode(Node* root, Node* dNode) {
    queue<Node*> q;
    q.push(root);
    Node* temp;
    while (!q.empty()) {
        temp = q.front();
        q.pop();
        if (temp == dNode) {
            temp = nullptr;
            delete dNode;
            return;
        }
        if (temp->right) {
            if (temp->right == dNode) {
                temp->right = nullptr;
                delete dNode;
                return;
            }
            q.push(temp->right);
        }
        if (temp->left) {
            if (temp->left == dNode) {
                temp->left = nullptr;
                delete dNode;
                return;
            }
            q.push(temp->left);
        }
    }
}

Node* deleteNode(Node* root, int key) {
    if (root == nullptr) return nullptr;
    if (root->left == nullptr && root->right == nullptr) {
        if (root->data == key) {
            delete root;
            return nullptr;
        }
        return root;
    }

    queue<Node*> q;
    q.push(root);
    Node* temp = nullptr;
    Node* keyNode = nullptr;

    while (!q.empty()) {
        temp = q.front();
        q.pop();
        if (temp->data == key) {
            keyNode = temp;
        }
        if (temp->left) q.push(temp->left);
        if (temp->right) q.push(temp->right);
    }

    if (keyNode != nullptr) {
        int x = temp->data;
        deleteDeepestNode(root, temp);
        keyNode->data = x;
    }
    return root;
}

void inorderTraversal(Node* root) {
    if (root == nullptr) return;
    inorderTraversal(root->left);
    cout << root->data << " ";
    inorderTraversal(root->right);
}

int main() {
    Node* root = nullptr;
    int choice, data;

    do {
        cout << "\n--- MANIPULASI POHON BINER ---\n";
        cout << "1. Inisialisasi / Tambah Akar\n";
        cout << "2. Insert Simpul (Level Order)\n";
        cout << "3. Hapus Simpul\n";
        cout << "4. Tampilkan Pohon (Inorder)\n";
        cout << "5. Keluar\n";
        cout << "Pilihan: ";
        cin >> choice;

        switch (choice) {
            case 1:
                cout << "Masukkan data akar: ";
                cin >> data;
                if (root == nullptr) {
                    root = createNode(data);
                    cout << "Akar berhasil dibuat.\n";
                } else {
                    cout << "Akar sudah ada. Gunakan menu Insert.\n";
                }
                break;
            case 2:
                cout << "Masukkan data simpul baru: ";
                cin >> data;
                insertNode(root, data);
                cout << "Simpul berhasil ditambahkan.\n";
                break;
            case 3:
                if (root == nullptr) {
                    cout << "Pohon kosong.\n";
                } else {
                    cout << "Masukkan data simpul yang akan dihapus: ";
                    cin >> data;
                    root = deleteNode(root, data);
                    cout << "Simpul berhasil dihapus (jika ada).\n";
                }
                break;
            case 4:
                if (root == nullptr) {
                    cout << "Pohon kosong.\n";
                } else {
                    cout << "Hasil Inorder Traversal: ";
                    inorderTraversal(root);
                    cout << endl;
                }
                break;
            case 5:
                cout << "Program selesai.\n";
                break;
            default:
                cout << "Pilihan tidak valid.\n";
        }
    } while (choice != 5);

    return 0;
}