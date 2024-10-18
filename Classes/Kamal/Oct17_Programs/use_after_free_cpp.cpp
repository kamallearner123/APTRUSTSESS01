#include <iostream>
#include <memory>   // for std::unique_ptr
#include <string>

class MyData {
public:
    MyData(const std::string& info) : info(info) {
        std::cout << "Memory allocated for MyData with info: " << info << std::endl;
    }
    ~MyData() {
        std::cout << "Memory freed for MyData with info: " << info << std::endl;
    }

    void show() const {
        std::cout << "Data: " << info << std::endl;
    }

private:
    std::string info;
};

int main() {
    // Dynamically allocate memory using smart pointer (unique_ptr)
    std::unique_ptr<MyData> data = std::make_unique<MyData>("This is dynamically allocated memory");

    while (true) {
        std::cout << "Memory is allocated. ";
        data->show();

        // Prompt user to decide if they want to free the memory
        std::cout << "Do you want to free the allocated memory? (y/n): ";
        char choice;
        std::cin >> choice;

        if (choice == 'y') {
            // Free the memory by resetting the unique_ptr
            data.reset();
            std::cout << "Memory has been freed." << std::endl;
            std::unique_ptr<MyData> data2 = std::make_unique<MyData>("This is dynamically allocated memory");

        } else if (choice == 'n') {
            std::cout << "Memory not freed, continuing..." << std::endl;
        } else {
            std::cout << "Invalid option. Please enter 'y' or 'n'." << std::endl;
        }

    }

    std::cout << "Program finished." << std::endl;
    return 0;
}

