using namespace std;
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

int main() {
    ifstream inputFile("input.txt");
    if (!inputFile) {
        cerr << "Unable to open file" << endl;
        return 1;
    }

    string s;
    char c;
    while (inputFile.get(c)) {
        s += c;
    }

    cout << s << endl;

    return 0;
}