/* This is a program to display a hypothetical store in C++
*/

#include <iostream.h>
#include <iomanip.h>

int main (void) {
    // print to user
    cout << "\tPart Number\tQty On Hand";
    cout << "\tQty On Order\tPrice\n";
    
    // print details
    cout    << fixed    << showpoint    << setprecision(2);
    cout    << "\t  "   << setfill('0')
            << setw(6)   << 31235    << "\t"
            << setfill(' ')
            << setw(7)  <<  22  <<  "\t\t"
            << setw(7)  <<  86  <<  "\t\t"
            << '$'      << set(7)   << 45.62    << endl;
    cout    << "\t "    << setfill('0')
            << setw(6)  << 321  << "\t"
            << setfill(' ')
            << setw(7)  << 55   << "\t\t"
            << setw(7)  << 21   << "\t\t"
            << '$'  << setw(7)  << 122.     << endl;
    cout    << "\t "    << setfill('0')
            << setw(6)  << 28764  << "\t"
            << setfill(' ')
            << setw(7)  << 0   << "\t\t"
            << setw(7)  << 24   << "\t\t"
            << '$'  << setw(7)  << .75     << endl;
    cout    << "\t "    << setfill('0')
            << setw(6)  << 3232  << "\t"
            << setfill(' ')
            << setw(7)  << 12   << "\t\t"
            << setw(7)  << 0   << "\t\t"
            << '$'  << setw(7)  << 10.91     << endl;

    // closing statement
    cout    << "\n\tEnd of Report\n";

    // return
    return 0;
} // end of main