# WoW Addon Manager
A tool for automatically downloading, installing and updating World of Warcraft
addons.

## Disclaimer
So far, the application has only been tested on Windows 10 and only works with
Retail World of Warcraft (not with Classic).
Note that this project is still early in development and as such may be buggy.
Do not use the application if you're unsure.
If you do decide you want to use the application, I recommend making a backup of
your AddOns folder before first using it.

## Setup instructions
You can either get a built executable of the application, or get the source code
and compile it yourself.

### With a built executable
Go to the
[releases page](https://github.com/Kumodatsu/wow-addon-manager/releases).
Find the section of the version you want.
The latest version is always on top.
At the bottom of the section, there is a small "Assets" drop down.
Click on it, then click on "wow-addon-manager.zip" to download.

The downloaded zip file contains the executable of the application.
Place this executable anywhere you want.
I suggest making a new folder to keep it in to keep things organized when you
add your configuration file (see "Usage").

### Building the project yourself
The project is written in [Rust](https://www.rust-lang.org/),
and as such you need the Rust toolchain (cargo, rustc) to build the project.
You can get get it [here](https://www.rust-lang.org/learn/get-started).

To get the repository, use the following command:

    git clone https://github.com/Kumodatsu/wow-addon-manager.git

To build the project, go into the project folder (the one containing the
"Cargo.toml" file) and run

    cargo build

To run the project, use the command

    cargo run

You need to have a configuration file in the directory from which you run this
command, or the application will complain (see "Usage").

## Usage (0.1.0)
To use the program, you must create a "config.yaml" in the same folder as the
application (or in the project folder if you built it yourself).
Take care that you can see file name extensions when you do this, or you might
accidentally create a file with a name like "config.yaml.txt" (if you're using
Windows, in Windows Explorer click on "View" and then check "File name
extensions").
Open this file with a text editor.
Here you can specify which addons you want to download (or update) as well as
where your World of Warcraft AddOns folder is located.
Here is an example of a correctly formatted configuration file:

    path:
        C:\Program Files (x86)\World of Warcraft\_retail_\Interface\AddOns

    curseforge:
        - 290830
        - 311718
        - 75973

    github:
        - https://github.com/Kumodatsu/Invite123

The _path_ section is used to specify where your AddOns folder is located.
This is always your World of Warcraft directory, and then
`_retail_/Interface/AddOns`.
The _curseforge_ section is to specify addons that should be downloaded from
CurseForge.
The numbers are the addon's _Project ID_ which you can find on the its
CurseForge page.
The _github_ section is to specify addons that should be downloaded from GitHub.
Just specify the base URL to the repository here.
If you do not need a section, you can just leave it out altogether (for example,
if none of your addons are from GitHub, you can just leave the _github_ section
out).

Once you have your configuration file (make sure it's saved), simply run the
application.
It will then try to find the addons you specified online, download them, extract
them and finally move them to your AddOns folder.
How long this takes naturally depends on the number of addons, your download
speed and other such factors.
