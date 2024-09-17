#include <iostream>

using namespace std;
class my_vec {
    unsigned int len;
    unsigned int cap;
    char *data;
    my_vec(int data_len) {
        len = data_len;
        data = (char *)malloc(data_len);
    }
};

int main() {
    my_vec v1 = {100};
}
