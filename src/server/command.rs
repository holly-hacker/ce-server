#![allow(dead_code)]

pub type Command = u8;

pub const CMD_GETVERSION: Command = 0;
pub const CMD_CLOSECONNECTION: Command = 1;
pub const CMD_TERMINATESERVER: Command = 2;
pub const CMD_OPENPROCESS: Command = 3;
pub const CMD_CREATETOOLHELP32SNAPSHOT: Command = 4;
pub const CMD_PROCESS32FIRST: Command = 5;
pub const CMD_PROCESS32NEXT: Command = 6;
pub const CMD_CLOSEHANDLE: Command = 7;
pub const CMD_VIRTUALQUERYEX: Command = 8;
pub const CMD_READPROCESSMEMORY: Command = 9;
pub const CMD_WRITEPROCESSMEMORY: Command = 10;
pub const CMD_STARTDEBUG: Command = 11;
pub const CMD_STOPDEBUG: Command = 12;
pub const CMD_WAITFORDEBUGEVENT: Command = 13;
pub const CMD_CONTINUEFROMDEBUGEVENT: Command = 14;
pub const CMD_SETBREAKPOINT: Command = 15;
pub const CMD_REMOVEBREAKPOINT: Command = 16;
pub const CMD_SUSPENDTHREAD: Command = 17;
pub const CMD_RESUMETHREAD: Command = 18;
pub const CMD_GETTHREADCONTEXT: Command = 19;
pub const CMD_SETTHREADCONTEXT: Command = 20;
pub const CMD_GETARCHITECTURE: Command = 21;
pub const CMD_MODULE32FIRST: Command = 22;
pub const CMD_MODULE32NEXT: Command = 23;
pub const CMD_GETSYMBOLLISTFROMFILE: Command = 24;
pub const CMD_LOADEXTENSION: Command = 25;
pub const CMD_ALLOC: Command = 26;
pub const CMD_FREE: Command = 27;
pub const CMD_CREATETHREAD: Command = 28;
pub const CMD_LOADMODULE: Command = 29;
pub const CMD_SPEEDHACK_SETSPEED: Command = 30;
pub const CMD_VIRTUALQUERYEXFULL: Command = 31;
pub const CMD_GETREGIONINFO: Command = 32;
pub const CMD_COMMANDLIST2: Command = 255;
