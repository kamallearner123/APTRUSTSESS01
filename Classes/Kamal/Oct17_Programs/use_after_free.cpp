#include <iostream>
#include <string.h>
#include <unistd.h>

using namespace std;

struct cat {
    void (*speak)(void);
    int id;
};

struct dog {
    void (*speak)(void);
    int id;
};

void dog_speak() {
    cout << "bow" << endl;
}

void cat_speak() {
    cout << "mew" << endl;
}


struct dog *mydog;
struct cat *mycat;

int main() {
    while(1) {
        char input[32];
        scanf("%32s", input);

        if (strcmp("newdog", input) == 0) {
            mydog = (struct dog *)malloc(sizeof(struct dog));
            mydog->id = 0x1122334455667788;
            mydog->speak = dog_speak;
        } else if (strcmp("newcat", input) == 0) {
            mycat = (struct cat *)malloc(sizeof(struct dog));
            //mycat->id = 0x7777777777777777;
            //mycat->speak = cat_speak;
        } else if (strcmp("delcat", input) == 0) {
            free(mycat);
        } else if (strcmp("deldog", input) == 0) {
            free(mydog);
        } else {
            mydog->speak();
            mycat->speak();
        }
    }
}