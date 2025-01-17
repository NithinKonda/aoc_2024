using namespace std;
#include <iostream>
#include <fstream>
#include <vector>

int main() {
    ifstream inputFile("input.txt");
    if (!inputFile) {
        cerr << "Unable to open file" << endl;
        return 1;
    }

    int H = 140;
    vector<string> a(H);
    for (string& row : a) {
        if (!(inputFile >> row)) {
            cerr << "Error reading from file" << endl;
            return 1;
        }
    }



    

    // for (const string& row : a) {
    //     cout << row << endl;
    // }
    // cout << a.size();

    return 0;
}