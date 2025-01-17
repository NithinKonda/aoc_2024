using namespace std;
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>

int main() {
    ifstream inputfile("input.txt");
    if (!inputfile) {
        cerr << "unable to open file input.txt";
        return 1;
    }

    stringstream buffer;
    buffer << inputfile.rdbuf();
    string s = buffer.str();



    int answer = 0;
    
    int n = (int) s.size();

    for(int i=0;i<n;i++) {
        if(s[i] == 'm'){
            if(s[i+1]=='u' && s[i+2] == 'l' && s[i+3]=='(')
            {
                i+=4;


                int x= getNumber(i);
                if(s[i] == ',')
                {
                    i+=1;
                    int y = getNumber(i);


                    if(s[i]==')'){
                        if(x!=-1 && y!=-1) {
                            answer+=x*y;
                        }
                    }
                }
            }
        }
         
         cout<<answer<<endl;
    }




}