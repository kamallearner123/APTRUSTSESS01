#include <iostream>
using namespace std;

class Solution {
public:
    double myPow(double x, int n) {

        double result = 1;
        unsigned int p = abs(n);
        if (n==0 || abs(x)==1) {
            if (x<0 && n%2==1) {
                result *= -1;
            }
            return result;
        }

        while (p!=0) {
            result *= x;
            p -= 1;
        }

        if (n<0) {
            result = (1/result);
        }

        return result;        
    }
};


int main() {
    Solution s;
    cout << s.myPow(-3.0, -5) << endl;
}