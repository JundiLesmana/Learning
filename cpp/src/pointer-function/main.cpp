#include <iostream>
using namespace std;

void fungsi (int *b){
    cout << "alamat b: " << b << endl;
    cout << "nilai b: " << *b << endl;

}

void kuadrat (int *x){
    *x = (*x) * (*x);
}

int main(){

    int a = 10;

    cout << "nilai a: " << a << endl;
    cout << "alamat a: " <<&a << endl;

    fungsi(&a);
    kuadrat(&a);

    cout << "nilai a: " << a << endl;

    cin.get();
    return 0;
}