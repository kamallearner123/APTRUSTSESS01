
class MyVector {
private:
    int* buffer;        // Pointer to dynamically allocated array
    std::size_t capacity; // Total allocated memory
    std::size_t length;     // Number of elements in the vector

public:
    // Constructor
    MyVector() : data(nullptr), capacity(0), length(0) {}

    // Destructor to free allocated memory
    ~MyVector() {
        delete[] data;
    }

    // Function to add elements to the vector
    void push_back(int value);

    // Access an element at a given index (with bounds checking)
    int& operator[](std::size_t index);
};

