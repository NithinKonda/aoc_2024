using namespace std;

#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <map>

int main()
{
    std::ifstream inputFile("input.txt");

    if (!inputFile)
    {
        cerr << "Error opening file" << endl;
    }

    std::vector<int> a;
    std::vector<int> b;
    std::string line;

    while (std::getline(inputFile, line))
    {
        std::istringstream lineStream(line);
        int num1, num2;


        if (lineStream >> num1 >> num2)
        {
            a.push_back(num1); 
            b.push_back(num2); 
        }
    }


    inputFile.close();

    int sum =0;

    sort(a.begin(), a.end());
    sort(b.begin(), b.end());

    map<int,int> freq;
    for(int x:b) {
        freq[x]++;
    }


    for(int i=0;i<a.size();i++) {
        sum += a[i] * freq[a[i]];
    }

    cout << sum << endl;
}
