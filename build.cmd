@echo off

set ROOT=%~dp0
set SRCDIR=%ROOT%src
set BINDIR=%ROOT%bin
set OBJDIR=%ROOT%obj

if not exist %BINDIR% mkdir %BINDIR%

rustc -o %BINDIR%\reek.exe %SRCDIR%/reek/main.rs