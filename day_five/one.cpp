using namespace std;
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <cassert>
#include <algorithm>
#include <map>

int main() {
    string s;
    map<int,vector<int>> edges;
    while(getline(cin,s))
    {
        if(s.empty() || !isdigit(s[0])){
            continue;
        }

        int len = (int) s.length();


        vector<int> v;
        bool is_pipe = false;
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

                if (s[i] == '|')
                {
                    is_pipe = true;
                }


                assert(s[i] == ',' || s[i] == '|' || s[i]=='\n' || s[i]==0);
            }
        }
        if (was_pipe) {
			edges[v[0]].push_back(v[1]);
		}
   else {
			set<int> earlier;
			bool ok = true;
			for (int i = 0; i < (int) v.size(); i++) {
				int x = v[i];
				for (int y : edges[x]) {
					if (earlier.count(y)) {
						ok = false;
					}
				}
				earlier.insert(x);
			}
			if (ok) {
				answer += v[v.size()/2];
			}
		}
	}
	cout << answer << "\n";
}