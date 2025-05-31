#
# 'make'        build executable file 'main'
# 'make clean'  removes all .o and executable files
#

# define the Cpp compiler to use
CXX = g++-13

# define any compile-time flags
CXXFLAGS := -std=c++20 -Wall -Wextra -g -pthread 

# define library paths in addition to /usr/lib
#   if I wanted to include libraries not in /usr/lib I'd specify
#   their path using -Lpath, something like:
LFLAGS = -lpqxx -lpq -lcpprest -lpthread -lssl -lcrypto -lcppunit

# lib/app name
BIN_TYPE = exe
NEKOKAN_PACKAGE_NAME := nekokan_music
BIN_NAME := nkm

# define nekokan header dir
NEKOKAN_HEADER_DIR := $(NEKOKAN_LIB_DIR)/include

# define output directory
OUTPUT := output

# define source directory
SRC := src

# define include directory
INCLUDE := include $(NEKOKAN_HEADER_DIR) 

LIB	:= lib $(NEKOKAN_LIB_DIR)

ifeq ($(OS),Windows_NT)
MAIN := $(BIN_NAME).exe
SOURCEDIRS := $(SRC)
INCLUDEDIRS := $(INCLUDE)
LIBDIRS := $(LIB)
FIXPATH = $(subst /,\,$1)
RM := del /q /f
MD := mkdir
else
MAIN := $(BIN_NAME)
SOURCEDIRS := $(shell find $(SRC) -type d)
INCLUDEDIRS := $(shell find $(INCLUDE) -type d)
LIBDIRS := $(shell find $(LIB) -type d)
FIXPATH = $1
RM = rm -f
RMREC = rm -fR
MD := mkdir -p
CP := cp
FULLRECCP := cp -fR
LS := ls -al
endif

# define any directories containing header files other than /usr/include
INCLUDES := $(patsubst %,-I%, $(INCLUDEDIRS:%/=%))

# define the C libs
LIBS := $(patsubst %,-L%, $(LIBDIRS:%/=%))

# define the C source files
SOURCES := $(wildcard $(patsubst %,%/*.cpp, $(SOURCEDIRS)))

# define the C object files
OBJECTS := $(SOURCES:.cpp=.o)

# define the dependency output files
DEPS := $(OBJECTS:.o=.d)

ifeq ($(BIN_TYPE),exe)
INSTALL_PATH_PRT := $(NEKOKAN_BIN_DIR)/$(NEKOKAN_PACKAGE_NAME)
INCLUDE_CP_SRC := /tmp/dummy
INCLUDE_CP_DEST := /tmp/dummy2
else
INSTALL_PATH_PRT := $(NEKOKAN_LIB_DIR)
INCLUDE_CP_SRC := ./include/*
INCLUDE_CP_DEST := $(NEKOKAN_HEADER_DIR)
endif
INSTALL_PATH := $(INSTALL_PATH_PRT)/$(BIN_NAME)

#
# The following part of the makefile is generic; it can be used to
# build any executable just by changing the definitions above and by
# deleting dependencies appended to the file from 'make depend'
#

OUTPUTMAIN := $(call FIXPATH,$(OUTPUT)/$(MAIN))

all: $(OUTPUT) $(MAIN)
	echo Executing 'all' complete!

$(OUTPUT):
	$(MD) $(OUTPUT)

$(MAIN): $(OBJECTS)
	$(CXX) $(CXXFLAGS) $(INCLUDES) -o $(OUTPUTMAIN) $(OBJECTS) $(LFLAGS) $(LIBS)

# include all .d files
-include $(DEPS)

# this is a suffix replacement rule for building .o's and .d's from .c's
# it uses automatic variables $<: the name of the prerequisite of
# the rule(a .c file) and $@: the name of the target of the rule (a .o file)
# -MMD generates dependency output files same name as the .o file
# (see the gnu make manual section about automatic variables)
.cpp.o:
	$(CXX) $(CXXFLAGS) $(INCLUDES) -c -MMD $<  -o $@

.PHONY: clean
clean:
	$(RM) $(OUTPUTMAIN)
	$(RM) $(call FIXPATH,$(OBJECTS))
	$(RM) $(call FIXPATH,$(DEPS))
	@echo Cleanup complete!

.PHONY: install
install:
	$(MAKE) all
	$(RMREC) $(INSTALL_PATH)
	$(RMREC) $(NEKOKAN_HEADER_DIR)/$(NEKOKAN_PACKAGE_NAME)
	$(MD) $(INSTALL_PATH_PRT)
	$(MD) $(NEKOKAN_HEADER_DIR)/$(NEKOKAN_PACKAGE_NAME)
	$(CP) $(OUTPUTMAIN) $(INSTALL_PATH)
	$(FULLRECCP) $(INCLUDE_CP_SRC) $(INCLUDE_CP_DEST)
	$(LS) $(INSTALL_PATH_PRT)
	@echo install complete!

run: all
	./$(OUTPUTMAIN)
	@echo Executing 'run: all' complete!

