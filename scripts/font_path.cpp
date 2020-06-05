// source: https://gist.github.com/take-cheeze/727091/57f27649d73aa0296407947b63c0018eac2a247b

#include <X11/Xlib.h>

#include <cassert>
#include <cstdlib>

#include <algorithm>
#include <iostream>
#include <iterator>


int main()
{
	Display* d = XOpenDisplay(NULL); assert(d);
	int s = DefaultScreen(d);

	int fontListSize;
	char** fontList = XGetFontPath(d, &fontListSize); assert(fontList);

	std::copy( fontList, fontList + fontListSize,
		std::ostream_iterator<char const*>(std::cout, "\n") );

	XFreeFontPath(fontList);
	XCloseDisplay(d);

	return EXIT_SUCCESS;
}
