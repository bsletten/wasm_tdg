#
# Platform Independent
# Bitmap Image Reader Writer Library
# By Arash Partow - 2002
#
# URL: http://partow.net/programming/bitmap/index.html
#
# Copyright Notice:
# Free use of this library is permitted under the
# guidelines and in accordance with the most
# current version of the MIT License.
# http://www.opensource.org/licenses/MIT
#


COMPILER      = -em++
OPTIONS       = -ansi -pedantic-errors -Wall -Wall -Werror -Wextra -o
LINKER_OPT    = -L/usr/lib -lstdc++ -lm

all: bitmap_test

bitmap_test: bitmap_test.cpp bitmap_image.hpp
	$(COMPILER) $(OPTIONS) bitmap_test.js bitmap_test.cpp $(LINKER_OPT) -s FORCE_FILESYSTEM=1 -s ALLOW_MEMORY_GROWTH=1  -s INVOKE_RUN=0 -s EXTRA_EXPORTED_RUNTIME_METHODS="['callMain']"
valgrind_check:
	valgrind --leak-check=full --show-reachable=yes --track-origins=yes -v ./bitmap_test

clean:
	rm -f core *.o *.bak *stackdump *~ *.wasm

#
# The End !
#
