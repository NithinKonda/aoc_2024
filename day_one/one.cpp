using namespace std;

#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

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
    for (int i=0; i<a.size();i++)
    {
        sum += abs(a[i]-b[i]);
    }
    cout << sum << endl;
    
    return 0;
}
