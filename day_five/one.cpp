using namespace std;
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

int main() {
    string s;
    while(getline(cin,s))
    {
        if(s.empty() || !isdigit(s[0])){
            continue;
        }

        int len = (int) s.length();


        vector<int> v;
        for(int i=0;i<len;i++)
        {
            if(isdigit(s[i]))
            {
                int x=0;
                while(isdigit(s[i]))
                {
                    x=10*x+(s[i]-'0');
                    i++;
                }
                v.push_back(x);


                assert(s[i] == ',' || s[i] == '|' || s[i]=='\n' || s[i]==0);
            }
        }
    }

}