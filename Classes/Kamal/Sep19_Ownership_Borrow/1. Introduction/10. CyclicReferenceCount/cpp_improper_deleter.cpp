#include <iostream>
#include <memory>

class Resource {
public:
    Resource() {
        std::cout << "Resource acquired\n";
    }
    ~Resource() {
        std::cout << "Resource destroyed\n";
    }
};

void improper_deleter(Resource* res) {
    // This function does nothing (incorrect deleter), so the resource is never freed.
    std::cout << "Improper deleter called\n";
}

int main() {
    // Create a shared_ptr with an improper custom deleter
    std::shared_ptr<Resource> ptr(new Resource);

    std::cout << "End of main\n";
    // Resource won't be destroyed here
}
