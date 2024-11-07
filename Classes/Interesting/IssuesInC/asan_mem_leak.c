#include <stdio.h>
#include <stdlib.h>
#include <string.h>
 
int main(int argc, const char *argv[]) {
    char *s = malloc(100);
    strcpy(s, "Hello world!");
    printf("string is: %s\n", s);

    int a;
    if (a>10) {
        printf("a is greateer than 10");
    }
    return 0;
}