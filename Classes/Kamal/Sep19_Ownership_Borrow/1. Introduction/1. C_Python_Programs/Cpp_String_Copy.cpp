#include <iostream>
using namespace std;
int main() {
    string s = "It is Rust";
    string t = s;
    string u = s;

    t.push_back('!');
    u.push_back(';');
    cout << "s = "  << s << endl;
    cout << "t = "  << t << endl;
    cout << "u = "  << u << endl;
}


