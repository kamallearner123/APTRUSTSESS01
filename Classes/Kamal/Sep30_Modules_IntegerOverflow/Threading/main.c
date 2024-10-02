struct db {
    pthread_mutex *mut;
    char *data;
};

struct db mydb;

void fn1() {
    //pthread_mutex_lock(mydb.mut);
    /// operat on data
    myds.data// Allowed
    //pthread_mutex)unlock(); 
}

void fn2() {
    pthread_mutex_lock(mydb.mut);
    /// operat on data
    pthread_mutex)unlock(); 
}

int main() {
    //create dthreads
    create(t1);
    create(t2);
}