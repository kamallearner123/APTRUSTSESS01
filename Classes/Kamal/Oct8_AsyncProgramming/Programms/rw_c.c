#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_READERS 5
#define NUM_WRITERS 2

pthread_rwlock_t rwlock;   // Read-write lock
int shared_data = 0;       // Shared data between threads

void* reader(void* arg) {
    int reader_id = *((int*)arg);
    for (int i = 0; i < 5; ++i) {
        pthread_rwlock_rdlock(&rwlock);  // Acquire read lock
        printf("Reader %d: read shared_data = %d\n", reader_id, shared_data);
        shared_data++;
        pthread_rwlock_unlock(&rwlock);  // Release read lock
        sleep(1);
    }
    return NULL;
}

void* writer(void* arg) {
    int writer_id = *((int*)arg);
    for (int i = 0; i < 5; ++i) {
        pthread_rwlock_wrlock(&rwlock);  // Acquire write lock
        /*
            if (lokc_not_released) {
                thread_yield();
            }
        */
       
        spinlock(); 
        /*
            while(1) {
                kernel_check_lock();
            }        
        */

        shared_data += writer_id;        // Modify shared data
        printf("Writer %d: updated shared_data to %d\n", writer_id, shared_data);
        pthread_rwlock_unlock(&rwlock);  // Release write lock
        sleep(2);
    }
    return NULL;
}



int main() {
    pthread_t readers[NUM_READERS], writers[NUM_WRITERS];
    int reader_ids[NUM_READERS], writer_ids[NUM_WRITERS];

    pthread_rwlock_init(&rwlock, NULL);  // Initialize the rwlock

    // Create writer threads
    for (int i = 0; i < NUM_WRITERS; ++i) {
        writer_ids[i] = i + 1;
        pthread_create(&writers[i], NULL, writer, &writer_ids[i]);
    }

    // Create reader threads
    for (int i = 0; i < NUM_READERS; ++i) {
        reader_ids[i] = i + 1;
        pthread_create(&readers[i], NULL, reader, &reader_ids[i]);
    }

    // Wait for all writer threads to finish
    for (int i = 0; i < NUM_WRITERS; ++i) {
        pthread_join(writers[i], NULL);
    }

    // Wait for all reader threads to finish
    for (int i = 0; i < NUM_READERS; ++i) {
        pthread_join(readers[i], NULL);
    }

    pthread_rwlock_destroy(&rwlock);  // Destroy the rwlock
    return 0;
}
