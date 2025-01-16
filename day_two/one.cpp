#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

using namespace std;

bool isSafeReport(const std::vector<int>& report) {
    if (report.size() < 2) return false;
    
    bool increasing = true, decreasing = true;
    
    for (size_t i = 1; i < report.size(); ++i) {
        int diff = report[i] - report[i - 1];
        
         if (increasing && (diff <= 0 || diff > 3)) {
            increasing = false;
        }
        
     
        if (decreasing && (diff >= 0 || -diff > 3)) {
            decreasing = false;
        }
    }
    
    return increasing || decreasing;
}

int main() {
    std::ifstream inputFile("input.txt");
    if (!inputFile) {
        std::cerr << "Unable to open file" << std::endl;
        return 1;
    }

    std::string line;
    int safeReportCount = 0;

    while (std::getline(inputFile, line)) {
        std::vector<int> report;
        std::stringstream ss(line);
        int level;

        while (ss >> level) {
            report.push_back(level);
        }

        if (isSafeReport(report)) {
            ++safeReportCount;
        }
    }

    inputFile.close();  

    cout << "Number of safe reports: " << safeReportCount << endl;

    return 0;
}