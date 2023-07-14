echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
%CURDIR%\target\release\ectst.exe ecsignbase SECT163k1 1152 1152 2> %OUTFILE%
python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust