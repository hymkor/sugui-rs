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

### for scoop-installer

```
scoop install https://raw.githubusercontent.com/hymkor/sugui-rs/master/sugui.json
```

or

```
scoop bucket add hymkor https://github.com/hymkor/scoop-bucket
scoop install sugui
```
