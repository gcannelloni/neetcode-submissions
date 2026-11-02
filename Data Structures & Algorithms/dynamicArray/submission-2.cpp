#include<string>

class DynamicArray {

private:
    int* elements;
    int capacity;
    int currentSize;

public:

    DynamicArray(int capacity) {
        this->capacity = capacity;
        this->currentSize = 0;
        this->elements = (int *) malloc(sizeof(int) * capacity);
    }

    int get(int i) {
        return this->elements[i];
    }

    void set(int i, int n) {
        this->elements[i] = n;
    }

    void pushback(int n) {
        if(this->currentSize == this->capacity) this->resize();
        this->elements[currentSize] = n;
        currentSize += 1;
    }

    int popback() {
        currentSize -= 1;
        int el = this->elements[currentSize];
        this->elements[currentSize] = 0;
        return el;
    }

    void resize() {
        this->capacity *= 2;
        int* newElements = (int *) malloc(sizeof(int) * this->capacity);
        for (int i = 0; i < this->currentSize; i++) {
            newElements[i] = this->elements[i];
        }
        free(this->elements);
        this->elements = newElements;
    }

    int getSize() {
        return this->currentSize;
    }

    int getCapacity() {
        return this->capacity;
    }
};
