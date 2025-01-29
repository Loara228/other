#!/bin/bash

sudo apt install make gcc
make
./ciadpi -s1 -q1 -Y -At -f-1 -r1+s -As
