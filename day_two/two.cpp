#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

bool isSequenceSafe(const std::vector<int>& sequence) {
    if (sequence.size() < 2) return false;
    
    bool increasing = true, decreasing = true;
    
    for (size_t i = 1; i < sequence.size() && (increasing || decreasing); ++i) {
        int diff = sequence[i] - sequence[i-1];
        
        // Check increasing sequence
        if (increasing && (diff <= 0 || diff > 3)) {
            increasing = false;
        }
        
        // Check decreasing sequence
        if (decreasing && (diff >= 0 || -diff > 3)) {
            decreasing = false;
        }
    }
    
    return increasing || decreasing;
}

bool isSafeWithDampener(const std::vector<int>& report) {
    // First check if it's safe without removing any number
    if (isSequenceSafe(report)) {
        return true;
    }
    
    // Try removing each number one at a time
    for (size_t i = 0; i < report.size(); ++i) {
        std::vector<int> modified = report;
        modified.erase(modified.begin() + i);
        
        if (isSequenceSafe(modified)) {
            return true;
        }
    }
    
    return false;
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

        if (isSafeWithDampener(report)) {
            ++safeReportCount;
        }
    }

    inputFile.close();

    std::cout << "Number of safe reports with Problem Dampener: " << safeReportCount << std::endl;

    return 0;
}