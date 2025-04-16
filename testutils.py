#! /usr/bin/env python

import extargsparse
import sys
import os
import re
import logging
import struct


def format_bytes(inb,note=''):
    rets = '%s'%(note)
    idx = 0
    lastidx = 0
    for b in inb:
        if (idx % 16) == 0:
            if idx > 0:
                rets += '    '
                while lastidx != idx:
                    curv = inb[lastidx]
                    if curv >= ord(' ') and curv <= ord('~'):
                        rets += '%c'%(curv)
                    else:
                        rets += '.'
                    lastidx += 1
            rets += '\n0x%08x'%(idx)
        rets += ' 0x%02x'%(b)
        idx += 1

    if idx != lastidx:
        while (idx % 16) != 0:
            rets += '     '
            idx += 1
        rets += '    '
        while lastidx < len(inb):
            curv = inb[lastidx]
            if curv >= ord(' ') and curv <= ord('~'):
                rets += '%c'%(curv)
            else:
                rets += '.'
            lastidx += 1
        rets += '\n'
    return rets

def format_int_val(intv, note=''):
    sarr = []
    lastv = intv
    while lastv > 0:
        curv = lastv & 0xff
        sarr.insert(0,curv)
        lastv = lastv >> 8

    rets = '%s'%(note)
    idx = 0
    lastidx = 0
    while idx < len(sarr):
        if (idx % 16) == 0:
            if idx > 0:
                rets += '    '
                while lastidx != idx:
                    curv = sarr[lastidx]
                    if curv >= ord(' ') and curv <= ord('~'):
                        rets += '%c'%(curv)
                    else:
                        rets += '.'
                    lastidx += 1
            rets += '\n0x%08x'%(idx)
        rets += ' 0x%02x'%(sarr[idx])
        idx += 1

    if idx != lastidx:
        while (idx % 16) != 0:
            rets += '     '
            idx += 1
        rets += '    '
        while lastidx < len(sarr):
            curv = sarr[lastidx]
            if curv >= ord(' ') and curv <= ord('~'):
                rets += '%c'%(curv)
            else:
                rets += '.'
            lastidx += 1
        rets += '\n'
    return rets

def int_to_bytes(ints):
	retb = b''
	for i in ints:
		s = '%x'%(i)
		if (len(s) % 2) != 0:
			n = s[:1]
			v = int(n,16)
			retb += struct.pack('B',v)
			s = s[1:]
		while len(s) > 0:
			n = s[:2]
			v = int(n,16)
			retb += struct.pack('B',v)
			s = s[2:]
	return retb


def set_logging(args):
	loglvl= logging.ERROR
	if args.verbose >= 3:
		loglvl = logging.DEBUG
	elif args.verbose >= 2:
		loglvl = logging.INFO
	curlog = logging.getLogger(args.lognames)
	#sys.stderr.write('curlog [%s][%s]\n'%(args.logname,curlog))
	curlog.setLevel(loglvl)
	if len(curlog.handlers) > 0 :
		curlog.handlers = []
	formatter = logging.Formatter('%(asctime)s:%(filename)s:%(funcName)s:%(lineno)d<%(levelname)s>\t%(message)s')
	if not args.lognostderr:
		logstderr = logging.StreamHandler()
		logstderr.setLevel(loglvl)
		logstderr.setFormatter(formatter)
		curlog.addHandler(logstderr)

	for f in args.logfiles:
		flog = logging.FileHandler(f,mode='w',delay=False)
		flog.setLevel(loglvl)
		flog.setFormatter(formatter)
		curlog.addHandler(flog)
	for f in args.logappends:		
		if args.logrotate:
			flog = logging.handlers.RotatingFileHandler(f,mode='a',maxBytes=args.logmaxbytes,backupCount=args.logbackupcnt,delay=0)
		else:
			sys.stdout.write('appends [%s] file\n'%(f))
			flog = logging.FileHandler(f,mode='a',delay=0)
		flog.setLevel(loglvl)
		flog.setFormatter(formatter)
		curlog.addHandler(flog)
	return

def load_log_commandline(parser):
	logcommand = '''
	{
		"verbose|v" : "+",
		"logname" : "root",
		"logfiles" : [],
		"logappends" : [],
		"logrotate" : true,
		"logmaxbytes" : 10000000,
		"logbackupcnt" : 2,
		"lognostderr" : false
	}
	'''
	parser.load_command_line_string(logcommand)
	return parser


def read_file(infile=None):
    fin = sys.stdin
    if infile is not None:
        fin = open(infile,'rb')
    rets = ''
    if 'b' in fin.mode:
        rdata = b''
        while True:
            try:
                l = fin.read(64 * 1024)
                if l is None or len(l) == 0:
                    break
                rdata += l
            except:
                break
        if sys.version[0] == '3':
            rets = rdata.decode('utf-8',errors='ignore')
        else:
            rets = rdata
    else:        
        for l in fin:
            s = l
            rets += s

    if fin != sys.stdin:
        fin.close()
    fin = None
    return rets

def read_file_bytes(infile=None):
    fin = sys.stdin
    if infile is not None:
        fin = open(infile,'rb')
    retb = b''
    while True:
        if fin != sys.stdin:
            curb = fin.read(1024 * 1024)
        else:
            curb = fin.buffer.read()
        if curb is None or len(curb) == 0:
            break
        retb += curb
    if fin != sys.stdin:
        fin.close()
    fin = None
    return retb


def write_file(s,outfile=None):
    fout = sys.stdout
    if outfile is not None:
        fout = open(outfile, 'w+b')
    outs = s
    if 'b' in fout.mode:
        outs = s.encode('utf-8')
    fout.write(outs)
    if fout != sys.stdout:
        fout.close()
    fout = None
    return 

def write_file_bytes(sarr,outfile=None):
    fout = sys.stdout
    if outfile is not None:
        fout = open(outfile, 'wb')
    if 'b' not in fout.mode:
        fout.buffer.write(sarr)
    else:        
        fout.write(sarr)
    if fout != sys.stdout:
        fout.close()
    fout = None
    return 


def caprand_handler(args,parser):
	set_logging(args)
	matchexpr = re.compile('random\\s+number\\s+0x([0-9a-fA-F]+)',re.I)
	nums = []
	for f in args.subnargs:
		s = read_file(f)
		sarr = re.split('\n',s)
		lindx = 0
		for l in sarr:
			lindx += 1
			l = l.rstrip('\r')
			if len(l) == 0:
				continue
			m = matchexpr.findall(l)
			if m is not None and len(m) > 0:
				nums.append(int(m[0],16))
	outb = int_to_bytes(nums)
	for c in nums:
		logging.info('0x%x'%(c))
	logging.info('outb\n%s'%(repr(outb)))
	if args.output is not None:
		write_file_bytes(outb,args.output)


	sys.exit(0)
	return

def main():
    commandline='''
    {
        "input|i" : null,
        "output|o" : null,
        "caprand<caprand_handler>##to capture random##" : {
        	"$" : "+"
        }
    }
    '''
    parser = extargsparse.ExtArgsParse()
    load_log_commandline(parser)
    parser.load_command_line_string(commandline)
    parser.parse_command_line(None,parser)
    raise Exception('can not here for no command handle')
    return


if __name__ == '__main__':
    main()