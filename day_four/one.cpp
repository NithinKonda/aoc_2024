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
    const string TEMP = "XMAS";
    auto inside = [&](int row, int col)
    {
        return 0 <= row && row < H && 0 <= col && col < W;
    };

    for (int row = 0; row < H; row++)
    {
        for (int col = 0; col < W; col++)
        {
            if (a[row][col] == 'X')
            {
                for (int drow = -1; drow <= 1; drow++)
                {
                    for (int dcol = -1; dcol <= 1; dcol++)
                    {
                        if (drow == 0 && dcol == 0)
                        {
                            continue;
                        }
                        bool yes = true;

                        for (int i = 0; i < 4; i++)
                        {
                            int r2 = row + drow * i;
                            int c2 = col + dcol * i;
                            if (inside(r2, c2) && a[r2][c2] == TEMP[i])
                            {
                            }
                            else
                            {
                                yes = false;
                                break;
                            }
                        }
                        answer += yes;
                    }
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