all:
	gcc source.c -shared -o libclib1.dylib
install: all
	cp libclib1.dylib /usr/local/lib
clean:
	rm *.o *.a
