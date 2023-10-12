echo off

set CURDIR=%~dp0
set OUTFILE=z:\ownvfy.log
set SIMPLEOUT=z:\ownvfy_simple.log
set ECSIMPLE_LEVEL=50
REM set ECSIMPLE_RANDOP=z:\rand.bin
set ECPUBBIN=z:\own_ecpub.bin
set HASHNUM=0x99bcf1bc2a70d552e85a3b7efe51
set SIGNBIN=z:\own_sign.bin
set ECTYPE=secp112r1
REM set ECTYPE=SM2
pushd %CD% && cd %CURDIR% && cargo build --release && .\target\release\ectst.exe ecvfybase -vvvvv %ECTYPE% %ECPUBBIN% %HASHNUM% %SIGNBIN% 2> %OUTFILE% && python z:\pylib\utils.py -i %OUTFILE% -o %SIMPLEOUT% filterlog rust && popd
set ECSIMPLE_LEVEL=