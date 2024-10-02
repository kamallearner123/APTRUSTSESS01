struct sec_mes {
    int cap; // 4 bytes
    int len; // 4 bytes
    char *data; //pointer 8bytes
};

int main() {
    struct sec_mes s1;
    s1.data = malloc(100);
    s1.cap = 100;
    s1.len = 0;


    struct sec_mes *s2 = malloc(sizeof(struct sec_msg));
    s2->data = malloc(100);


    let s1 = String::from("hekfdsfsdfads"); //Stack 24 bytes + Heap 100 bytes
    let s2 = Box::new(String); // Stack 8bytes + Heap 24 bytes+100bytes
}


