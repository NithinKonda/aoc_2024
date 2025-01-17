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




    auto getNumber = [&](int &i){
        int x=0;
        while (x<1000 && isdigit(s[i])){

            x = 10*x+(s[i] - '0');
            i++;
        }

        if( 1<=x && x<=999) {
            return x;
        }
        return -1;
        
        
    };


    bool enable = true;

    for(int i=0;i<n-7;i++) {


        if (s.substr(i,4) == "do()") {
            enable= true;
        }
        if(s.substr(i,7)=="don\'t"){
            enable = false;
        }

        if(enable && s[i] == 'm'){
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
         
    }


         cout<<answer<<endl;


}