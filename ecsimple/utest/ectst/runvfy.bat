echo off

set CURDIR=%~dp0
set OUTFILE=z:\rust.log
set SIMPLEOUT=z:\rust_simple.log
set ECSIMPLE_LEVEL=50
set ECSIMPLE_RANDOP=z:\rand.bin
set ECPUBBIN=z:\ecpub.bin
set HASHNUM=7201
set SIGNBIN=z:\sign.bin
REM set ECTYPE=SECP112r1
set ECTYPE=c2pnb368w1
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecvfybase -vvvvv %ECTYPE% %ECPUBBIN% %HASHNUM% %SIGNBIN% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd