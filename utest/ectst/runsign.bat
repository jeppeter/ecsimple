echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
set PRIVNUMB=0x13c5873c53d1046528aeed5cbe4b
REM set ECTYPE=SECP112r1
set ECTYPE=secp112r1
set HASHNUM=0x99bcf1bc2a70d552e85a3b7efe51
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecsignbase %ECTYPE% %PRIVNUMB% %HASHNUM% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd
set ECSIMPLE_LEVEL=