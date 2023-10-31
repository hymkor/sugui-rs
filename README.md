sugui
=====

Start programs with administrator privileges without creating extra terminal windows
on Microsoft Windows.

- Compact ( 170 KBytes )
- Written in Rust
- It requires VCRUNTIME140.DLL

```
sugui "PROGRAM-PATH" "PARAMETERS"
```

Install
-------

Download the binary package from [Releases](https://github.com/hymkor/sugui-rs/releases) and extract the executable.

Install with scoop-instabler is avaliable, but there exists the problem
that the scoop-wrapper program `%USERPROFILE%\scoop\shims\sugui.exe`
creates a console window.

