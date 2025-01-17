using namespace std;
#include <iostream>
#include <fstream>
#include <vector>

int main()
{
    ifstream inputFile("input.txt");
    if (!inputFile)
    {
        cerr << "Unable to open file" << endl;
        return 1;
    }



const vector<pair<int,int>> dirs={{-1,-1},{-1,1},{1,1},{1,-1}};
    int H = 140;
    vector<string> a(H);
    for (string &row : a)
    {
        if (!(inputFile >> row))
        {
            cerr << "Error reading from file" << endl;
            return 1;
        }
    }
    int answer = 0;

    int W = a[0].length();

    for (int row = 1; row < H-1; row++)
    {
        for (int col = 1; col < W-1; col++)
        {
            if (a[row][col] == 'A')
            {
                string s;
                for(pair<int,int> dir:dirs){
                    s+=a[row+dir.first][col+dir.second];
                }

                if(s=="MMSS" || s=="MSSM" || s=="SSMM" || s=="SMMS"){
                    answer++;
                }
            }
        }
    }
        cout<<answer<<endl;
            // for (const string& row : a) {
            //     cout << row << endl;
            // }
            // cout << a.size();

            return 0;
}