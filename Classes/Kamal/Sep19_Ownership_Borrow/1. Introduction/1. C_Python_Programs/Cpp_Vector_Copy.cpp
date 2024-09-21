#include <iostream>
#include <vector>
using namespace std;
int main() {
    vector<int> s = {1,2,3};
    vector<int> t = s;
    vector<int> u = s;

    t.push_back(4);
    u.push_back(5);
    for (auto num:s)
        cout << "s = "  << num << endl;
    for (auto num:t)
        cout << "t = "  << num << endl;
    for (auto num:u)
    cout << "u = "  << num << endl;
}

