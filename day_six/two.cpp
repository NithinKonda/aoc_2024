#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> dirs = {{-1,0}, {0,1}, {1,0}, {0,-1}};


int main() {
	int H = 130;
	vector<string> a(H);
	for (int i = 0; i < H; i++) {
		cin >> a[i];
	}
	int W = a[0].length();
	
	pair<int,int> start_me{-1, -1};
	for (int row = 0; row < H; row++) {
		for (int col = 0; col < W; col++) {
			if (a[row][col] == '^') {
				start_me = {row, col};
				a[row][col] = '.';
			}
		}
	}

    auto solveCycle = [&]() {
		pair<int,int> me = start_me;
		int dir = 0;
		
		vector<bool> vis(H * W * 4);
    
}