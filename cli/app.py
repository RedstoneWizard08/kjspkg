#!/usr/bin/python3

# IMPORTS

# Built-in modules
from os import path, remove, getcwd, makedirs, walk, chmod, chdir, system, getenv, listdir # Working with files and system stuff
from os import name as osname # Windows/linux
from shutil import rmtree, move, copy, copytree # More file stuff
from pathlib import Path # EVEN MORE FILE STUFF
from tempfile import gettempdir # Get tmp dir of current os
from subprocess import run, DEVNULL # Subprocesses
from json import dump, load, dumps, loads # Json
from multiprocessing import Process # Async threads
from signal import signal, SIGTERM # Signals in threads
from time import sleep # Honk mimimimimimimi
from zipfile import ZipFile # Working with .jars
from itertools import zip_longest # Ziiiiiiiiiiiiiiiiiiiiiip

from http import server # Discord login stuff
from urllib.parse import urlparse, parse_qs # Parse url path

from stat import S_IWRITE # Windows stuff
from warnings import filterwarnings # Disable the dumb fuzz warning

from random import choice # Random splash

# External libraries
from fire import Fire # CLI tool

from requests import get, put, exceptions # Requests
from git import Repo, GitCommandNotFound, GitCommandError # Git cloning

from psutil import process_iter # Get processes
from flatten_json import flatten # Flatten dicts
from toml import loads as tomlload # Read .tomls
from esprima import parse, error_handler # Parse .js files

# CONSTANTS
VERSIONS = { # Version and version keys
    "1.12.2": 2,
    "1.12": 2,

    "1.16.5": 6,
    "1.16": 6,

    "1.18.2": 8,
    "1.18": 8,

    "1.19.2": 9,
    "1.19.3": 9,
    "1.19.4": 9,
    "1.19": 9,

    "1.20.1": 10,
    "1.20.2": 10,
    "1.20.3": 10,
    "1.20.4": 10,
    "1.20": 10,

    "1.21.1": 11,
    "1.21": 11
}
SCRIPT_DIRS = ("server_scripts", "client_scripts", "startup_scripts") # Script directories
ASSET_DIRS = ("data", "assets") # Asset directories
CONFIG = { # Default config
    "_": "Please, do not delete this file or any other file/directory labeled \".kjspkg\", even if they might seem empty and useless, they are still required. Thanks for your understanding!",

    "installed": {},
    "trustgithub": False
}
NL = "\n" # Bruh
LOGO = """
⠀⠀⠀⠀⢀⣤⣶⣿⣿⣶⣤⡀⠀⠀⠀⠀
⠀⠀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⠀⠀
⢠⣄⡀⠉⠻⢿⣿⣿⣿⣿⡿⠟⠉⢀⣠⡄
⢸⣿⣿⣷⣦⣀⠈⠙⠋⠁⣀⣴⣾⣿⣿⡇
⢸⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⡇
⢸⣿⣿⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⣿⣿⡇
⠀⠙⠻⣿⣿⣿⣿⠀⠀⣿⣿⣿⣿⠟⠋⠀
⠀⠀⠀⠀⠉⠻⢿⠀⠀⡿⠟⠉⠀⠀⠀⠀
""" # Epic logo in ascii

# VARIABLES
kjspkgfile = {} # .kjspkg file

# CLASSES
# class HTTPDiscordLoginRequestHandler(server.BaseHTTPRequestHandler):
#     def log_message(self, format, *args): return
#     def do_GET(self):
#         self.send_response(200)
#         self.send_header("Content-type", "text/html")
#         self.end_headers()
#         self.wfile.write(
# b"""<html>
#     <head>
#         <script>
#             close();
#         </script>
#     </head>
# </html>"""
#         )

#         token = post("https://discord.com/api/v10/oauth2/token", data={
#             "client_id": "1108295881247166496",
#             "client_secret": "yopSwUCpisfib6RR3aqHyp293vOyG4F5",
#             "grant_type": "authorization_code",
#             "code": parse_qs(urlparse(self.path).query)["code"][0],
#             "redirect_uri": "http://localhost:1337"
#         }).json()["access_token"]

#         print(post("https://discord.com/api/v10/channels/303440391124942858/messages", headers={
#             "Authorization": token
#         }).text)

# HELPER FUNCTIONS
def _bold(s:str) -> str: return "\u001b[1m"+s+"\u001b[0m" # Make the text bold
def _textbg(s:str) -> str: return "\u001b[47m\u001b[30m"+s+"\u001b[0m" # Make the text black and have white background
def _purple(s:str) -> str: return "\u001b[35;1m"+s+"\u001b[0m" # Make the text bright ourple
def _err(err:str, dontquit:bool=False): # Handle errors
    print("\u001b[31;1m"+err+"\u001b[0m") # Print error
    if not dontquit: exit(1) # Quit
def _carbon_err(): _err("CarbonJS has been abandoned (https://github.com/malezjaa/carbonjs/blob/main/README.md), and thus the support for its package format was removed from KJSPKG.") # Print carbon removal error
def _remove_prefix(pkgname:str) -> str: return pkgname.split(":")[-1] # Remove prefix
def _format_github(pkgname:str) -> str: return _remove_prefix(pkgname).split('/')[-1].split("@")[0].split("$")[0] # Remove github author, path and branch
def _loading_anim(prefix:str=""): # Loading animation
    loading = "⡆⠇⠋⠙⠸⢰⣠⣄" # Animation frames
    i = 0

    # Termination
    def terminate(*args): print(" "*(len(prefix)+2), end="\r"); exit(0)
    signal(SIGTERM, terminate)

    try:
        while 1:
            print(_bold(prefix+" "+loading[i % len(loading)]), end="\r") # Print the next anim frame
            i += 1
            sleep(.1)
    except KeyboardInterrupt: terminate() # Terminate on keyboard interrupt
def _loading_thread(*args) -> Process:  # Loading animation thread
    thread = Process(target=_loading_anim, args=args) # Create a thread
    thread.start() # Start it
    return thread # Return it
def _dumbass_windows_path_error(f, p:str, e): chmod(p, S_IWRITE) # Dumbass windows path error
def _check_for_fun(): return getenv("NO_FUN_ALLOWED") == None # Disable easter eggs

# TMP HELPER FUNCTIONS
def _create_tmp(pathintmp:str) -> str: # Create a temp directory and return its path
    tmppath = path.join("tmp", pathintmp)
    makedirs(tmppath, exist_ok=True)
    return tmppath
def _clear_tmp(): # Clear tmp directory
    if path.exists("tmp"): rmtree("tmp", onerror=_dumbass_windows_path_error) # Delete if exists

# PROJECT HELPER FUNCTIONS
def _check_project() -> bool: # Check if the current directory is a kubejs directory
    for dir in SCRIPT_DIRS:
        if path.exists(dir) and path.basename(getcwd())=="kubejs": return True
    return False
def _create_project_directories():
    for dir in SCRIPT_DIRS+ASSET_DIRS: makedirs(dir, exist_ok=True)  # Create asset and script directories
    for dir in SCRIPT_DIRS: makedirs(path.join(dir, ".kjspkg"), exist_ok=True)  # Create .kjspkg directories
def _project_exists() -> bool: return path.exists(".kjspkg") # Check if a kjspkg project exists
def _delete_project(): # Delete the project and all of the files
    for pkg in list(kjspkgfile["installed"].keys()): _remove_pkg(pkg, True) # Remove all packages
    for dir in SCRIPT_DIRS: rmtree(path.join(dir, ".kjspkg"), onerror=_dumbass_windows_path_error) # Remove .kjspkg dirs
    remove(".kjspkg") # Remove .kjspkg file
def _update_manifest(): # Update .kjspkg file
    global kjspkgfile

    # Update the config by adding keys that don't exist
    for k, v in CONFIG.items():
        if k not in kjspkgfile.keys(): kjspkgfile[k] = v
def _enable_reflection(): # Enable reflection on 1.16
    with open(path.join("config", "common.properties"), "a+") as f:
        if ("invertClassLoader=true" not in f.read().splitlines()): f.write("invertClassLoader=true")
def _check_for_forge(): return kjspkgfile["modloader"]=="forge" # Checks the project for forge
def _get_mod_manifest(modpath:str) -> dict: # Get a mod's mods.toml/fabric.mod.json
    modfile = ZipFile(path.join(getcwd(), "..", "mods", modpath))

    try:
        if _check_for_forge(): return tomlload(modfile.open("META-INF/mods.toml").read().decode("utf-8"))
        else: return loads(modfile.open("fabric.mod.json").read().decode("utf-8"))
    except KeyError: return # Check for wierd mods with no mods.toml/fabric.mod.json
def _get_mod_version(modpath:str) -> str:
    manifest = _get_mod_manifest(modpath) # Get manifest
    if manifest==None: return # Return none if not found

    if _check_for_forge(): return manifest["mods"][0]["version"]
    else: return manifest["version"]
def _get_versions() -> list: # Get all mod versions
    modversions = {}
    for i in listdir(path.join(getcwd(), "..", "mods")):
        if i.endswith(".jar"): 
            modversion = _get_mod_version(i)
            if modversion: modversions[_get_modid(i)] = modversion # For each mod file, get the mod version and add the mod id - mod version pair to the dict

    return modversions # Return the dict of mod versions  
def _get_modid(modpath:str) -> str: # Get mod id from a mod file
    manifest = _get_mod_manifest(modpath) # Get manifest
    if manifest==None: return # Return none if not found

    if _check_for_forge(): return manifest["mods"][0]["modId"]
    else: return manifest["id"]
def _get_modids() -> list: # Get all mod ids
    modids = []
    for i in listdir(path.join(getcwd(), "..", "mods")):
        if i.endswith(".jar"): 
            modid = _get_modid(i)
            if modid: modids.append(modid) # For each mod file, get the mod id and append

    return modids # Return the list of modids    

# def _discord_login(): # Login with discord for discord prefixes
#     server.HTTPServer(("", 1337), HTTPDiscordLoginRequestHandler).handle_request()

# PKG HELPER FUNCTIONS
def _reload_pkgs(): # Reload package registry cache
    with open(path.join(gettempdir(), "kjspkgs.json"), "w") as tmp: tmp.write(get(f"https://raw.githubusercontent.com/Modern-Modpacks/kjspkg/main/pkgs.json").text)
def _pkgs_json() -> dict: # Get the pkgs.json file
    pkgspath = path.join(gettempdir(), "kjspkgs.json")
    if not path.exists(pkgspath): _reload_pkgs()
    return load(open(pkgspath))

def _pkg_info(pkg:str, ghinfo:bool=True, refresh:bool=True) -> dict: # Get info about the pkg
    if refresh: _reload_pkgs() # Refresh pkgs

    prefix = pkg.split(":")[0] if len(pkg.split(":"))>1 else "kjspkg" # kjspkg: - default prefix
    packagename = _remove_prefix(pkg) # Get package name without the prefix

    # Call correct function based on prefix
    if prefix=="kjspkg": info = _kjspkginfo(packagename)
    elif prefix in ("carbon", "carbonjs"): _carbon_err() # info = _carbonpkginfo(packagename)
    elif prefix in ("github", "external"): info = _githubpkginfo(packagename)
    # elif prefix=="discord": 
    #     _discord_login()
    #     exit()
    else: _err("Unknown prefix: "+_bold(prefix))

    # Get github repo info if requested
    if info!=None and ghinfo:
        req = get(f"https://api.github.com/repos/{info['repo']}?ref={info['branch']}", headers=({"Authorization": "Bearer "+getenv("GITHUB_API_KEY")} if getenv("GITHUB_API_KEY") else {}))
        if req.status_code==200: info["ghdata"] = req.json()

    return info
def _kjspkginfo(pkg:str) -> dict: # Get info about a default kjspkg pkg
    pkgregistry = _pkgs_json() # Get the pkgs.json file
    if pkg not in pkgregistry.keys(): return # Return nothing if the pkg doesn't exist

    repo = pkgregistry[pkg] # Get the repo with branch
    branch = "main" # Set the branch to main by default
    if "@" in repo: # If the branch is specifed
        branch = repo.split("@")[-1] # Set the branch
        repo = repo.split("@")[0] # Remove the branch from the repo
    path = "."
    if "$" in repo: 
        path = repo.split("$")[-1] # Set the path
        repo = repo.split("$")[0] # Remove the path from the repo

    package = get(f"https://raw.githubusercontent.com/{repo}/{branch}{'/'+path if path!='.' else ''}/.kjspkg").json() # Get package info

    package["repo"] = repo # Add the repo to info
    package["branch"] = branch # Add the branch to info
    package["path"] = path # Add the path to info

    return package # Return the json object
# def _carbonpkginfo(pkg:str) -> dict: # Get info about a carbonjs pkg (https://github.com/malezjaa/carbonjs)
#     allpackages = get("https://carbon.beanstech.tech/api/packages").json() # Get all packages
#     allpackages = [i for i in allpackages if i["name"]==pkg.lower()] # Find the package with the name provided
#     if len(allpackages)==0: return # If not found, return nothing

#     repository = allpackages[0]['repository'].replace('https://github.com/', '') # Format the repository
#     branch = get(f"https://api.github.com/repos/{repository}").json()["default_branch"] # Get the default branch
#     info = get(f"https://raw.githubusercontent.com/{repository}/{branch}/carbon.config.json").json() # Request info about the package

#     return { # Return formatted info
#         "author": info["author"],
#         "description": info["description"],
        
#         "versions": list(dict.fromkeys([VERSIONS[i] for i in info["minecraftVersion"]])),
#         "modloaders": info["modloaders"],
#         "dependencies": [],
#         "incompatibilities": [],

#         "repo": repository,
#         "branch": branch,
#         "path": "."
#     }
def _githubpkginfo(pkg:str) -> dict: # Get dummy info about an external pkg
    return {
        "author": pkg.split("/")[0],
        "description": "",
        
        "versions": [kjspkgfile["version"]],
        "modloaders": [kjspkgfile["modloader"]],
        "dependencies": [],
        "incompatibilities": [],

        "repo": pkg,
        "branch": "main" if "@" not in pkg else pkg.split("@")[-1]
    }
def _move_pkg_contents(pkg:str, tmpdir:str, furtherpath:str): # Move the contents of the pkg to the .kjspkg folders
    # Find the license
    licensefile = path.join(tmpdir, "LICENSE") 
    if not path.exists(licensefile): licensefile = path.join(tmpdir, "LICENSE.txt")
    if not path.exists(licensefile): licensefile = path.join(tmpdir, "LICENSE.md")

    for dir in SCRIPT_DIRS: # Clone scripts & licenses into the main kjs folders
        tmppkgpath = path.join(tmpdir, furtherpath, dir)
        finalpkgpath = path.join(dir, ".kjspkg", pkg)
        if path.exists(tmppkgpath):
            move(tmppkgpath, finalpkgpath) # Files
            if path.exists(licensefile): copy(licensefile, finalpkgpath) # License

    assetfiles = [] # Pkg's asset files
    for dir in ASSET_DIRS: # Clone assets
        tmppkgpath = path.join(tmpdir, furtherpath, dir) # Get asset path
        if not path.exists(tmppkgpath): continue

        for dirpath, _, files in walk(tmppkgpath): # For each file in assets/data
            for name in files:
                tmppath = path.join(dirpath, name)
                patharray = tmppath.split(path.sep)
                finalpath = path.sep.join(patharray[patharray.index(dir):])

                makedirs(path.sep.join(finalpath.split(path.sep)[:-1]), exist_ok=True) # Create parent dirs
                move(tmppath, finalpath) # Move it to the permanent dir
                assetfiles.append(finalpath) # Add it to assetfiles
    
    kjspkgfile["installed"][pkg] = assetfiles # Add the pkg to installed
def _install_pkg(pkg:str, update:bool, quiet:bool, skipmissing:bool, reload:bool, *, _depmode:bool=False): # Install the pkg
    if not update and _format_github(pkg) in kjspkgfile["installed"]:  # If the pkg is already installed and the update parameter is false, notify the user and just return
        if not quiet: print(_bold(f"Package \"{pkg}\" already installed ✓"))
        return
    if update: 
        if pkg=="*": # If updating all packages
            for p in list(kjspkgfile["installed"].keys()): _install_pkg(p, True, quiet, skipmissing, reload, _depmode=True) # Update all packages
            return

        _remove_pkg(pkg, False) # If update is true, remove the previous version of the pkg

    package = _pkg_info(pkg, False, reload) # Get pkg
    if not package and reload: 
        _reload_pkgs() # Reload if not found
        package = _pkg_info(pkg, False, reload) # Try to get the pkg again

    if not package: # If pkg doesn't exist after reload
        if not skipmissing: _err(f"Package \"{pkg}\" does not exist") # Err
        else: return # Or just ingore

    pkg = _remove_prefix(pkg) # Remove pkg prefix

    # Unsupported version/modloader errs
    if _depmode and (kjspkgfile["version"] not in package["versions"] or kjspkgfile["modloader"] not in package["modloaders"]): return
    if kjspkgfile["version"] not in package["versions"]: _err(f"Unsupported version 1.{10+kjspkgfile['version']} for package \"{pkg}\"")
    if kjspkgfile["modloader"] not in package["modloaders"]: _err(f"Unsupported modloader \"{kjspkgfile['modloader'].title()}\" for package \"{pkg}\"")

    # Install dependencies & check for incompats
    modids = []
    if (("dependencies" in package.keys() and any([i.startswith("mod:") for i in package["dependencies"]])) or ("incompatibilities" in package.keys() and any([i.startswith("mod:") for i in package["incompatibilities"]]))): modids = _get_modids() # Get a list of all mod ids

    if "dependencies" in package.keys():
        for dep in package["dependencies"]: 
            if dep.lower().startswith("mod:") and _remove_prefix(dep.lower()) not in modids: _err(f"Mod \"{_remove_prefix(dep.replace('_', ' ').replace('-', ' ')).title()}\" not found.") # Check for mod dependency
            elif not dep.lower().startswith("mod:"): _install_pkg(dep.lower(), dep.lower() in kjspkgfile["installed"], quiet, skipmissing, reload) # Install/update package dependency
    if "incompatibilities" in package.keys(): 
        for i in package["incompatibilities"]:
            if i.lower().startswith("mod:") and _remove_prefix(i.lower()) in modids: _err(f"Incompatible mod: "+_remove_prefix(i.replace('_', ' ').replace('-', ' ')).title()) # Check for mod incompats
            elif i in kjspkgfile["installed"].keys(): _err(f"Incompatible package: "+i) # Throw err if incompats detected

    if not quiet: loadthread = _loading_thread(f"{'Installing' if not update else 'Updating'} {_format_github(pkg)}...") # Start the loading animation

    tmpdir = _create_tmp(pkg) # Create a temp dir
    try: Repo.clone_from(f"https://github.com/{package['repo']}.git", tmpdir, branch=package["branch"]) # Install the repo into the tmp dir
    except GitCommandError: Repo.clone_from(f"https://github.com/{package['repo']}.git", tmpdir) # If the branch is not found, try to install from the default one
    furtherpath = path.sep.join(package["path"].split("/")) # Set the furtherpath to the package path

    pkg = _format_github(pkg) # Remove github author and branch if present

    _move_pkg_contents(pkg, tmpdir, furtherpath) # Move the contents of the pkg to the kubejs folder
    put(f"https://tizudev.vercel.app/automatin/api/1025316079226064966/kjspkg?stat=downloads&id={pkg}") # Add 1 to the download counter on tizu's backend

    if not quiet:
        loadthread.terminate() # Kill the loading animation
        if pkg=="*": print(_bold(f"All packages updated succesfully! ✓")) # Show message if all packages are updated
        else: print(_bold(f"Package \"{_format_github(pkg)}\" {'installed' if not update else 'updated'} succesfully! ✓")) # Show message if one package is installed/updated
    
def _remove_pkg(pkg:str, skipmissing:bool): # Remove the pkg
    if pkg not in kjspkgfile["installed"].keys():
        if not skipmissing: _err(f"Package \"{pkg}\" is not installed") # If the pkg is not installed, err
        else: return # Or just ignore

    for dir in SCRIPT_DIRS: # Remove script files
        scriptpath = path.join(dir, ".kjspkg", pkg)
        if path.exists(scriptpath): rmtree(scriptpath, onerror=_dumbass_windows_path_error)
    for file in kjspkgfile["installed"][pkg]: # Remove asset files
        if path.exists(file): remove(file)

    del kjspkgfile["installed"][pkg] # Remove the pkg from installed

# MAIN COMMAND FUNCTIONS
def install(*pkgs:str, update:bool=False, quiet:bool=False, skipmissing:bool=False, reload:bool=True): # Install pkgs
    if update and not pkgs: pkgs = ("*",) # If update is passed and not pkgs are, set them to all pkgs

    # Install each given package
    for pkg in pkgs:
        pkg = pkg.lower() # Format

        if (pkg.startswith("github:")): # If the package is external
            if (
                ("trustgithub" in kjspkgfile.keys() and kjspkgfile["trustgithub"]) or # And external packages are trusted
                (quiet or input(_bold(f"Package \"{_format_github(pkg)}\" uses the \"github:\" prefix, external packages coming from github are not tested and are not guaranteed to work, do you want to disable external packages in this project? (Y/n) ")).lower()!="n") # Or quiet mode is set/prompt is answered with y
            ): kjspkgfile["trustgithub"]=True # Trust external packages
            else: # Otherwise
                kjspkgfile["trustgithub"]=False # Don't trust them
                if skipmissing: continue # Skip if skipmissing
                else: return # Stop if no skipmissing

        if update and pkg not in kjspkgfile["installed"].keys() and not skipmissing and pkg!="*": _err(f"Package \"{_format_github(pkg)}\" not found") # Err if package not found during update
        _install_pkg(pkg, update, quiet, skipmissing, reload) # Install package
        
def removepkg(*pkgs:str, quiet:bool=False, skipmissing:bool=False): # Remove pkgs
    for pkg in pkgs:
        pkg = _remove_prefix(pkg.lower())
        _remove_pkg(pkg, skipmissing)
        if not quiet: print(_bold(f"Package \"{pkg}\" removed succesfully! ✓"))
def update(*pkgs:str, **kwargs): # Update pkgs
    install(*pkgs, update=True, **kwargs)
def updateall(**kwargs): # Update all pkgs
    update("*", **kwargs)
def listpkgs(*, count:bool=False): # List pkgs
    if count: # Only show the pkg count if the "count" option is passed
        print(len(kjspkgfile["installed"].keys()))
        return

    if (len(kjspkgfile["installed"].keys())==0 and _check_for_fun()): # Easter egg if 0 pkg installed
        print(_bold("*nothing here, noone around to help*"))
        return

    print("\n".join(kjspkgfile["installed"].keys())) # Print the list
def pkginfo(pkg:str, *, script:bool=False, githubinfo:bool=True): # Print info about a pkg
    if(pkg.startswith("github:")): _err(f"Can't show info about an external package \"{_format_github(pkg)}\"") # Err if pkg is external

    info = _pkg_info(pkg, githubinfo, True) # Get the info
    if not info: _err(f"Package {pkg} not found") # Err if pkg not found

    # Tizu lookup view/download data
    headers = {"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; Unactivated)"} if _check_for_fun() else None
    downloaddata = get("https://tizudev.vercel.app/automatin/api/1025316079226064966/kjspkg?stat=downloads", headers=headers)
    viewdata = get("https://tizudev.vercel.app/automatin/api/1025316079226064966/kjspkg?stat=views", headers=headers)
    if viewdata.status_code==200:
        info["lookupapi"] = {
            "downloads": downloaddata.json()[pkg] if pkg in downloaddata.json().keys() else 0,
            "views": viewdata.json()[pkg] if pkg in viewdata.json().keys() else 0
        }

    # Print it (scripty)
    if script:
        print(dumps(info))
        return

    # Print it (pretty)
    print(f"""
{_bold(_remove_prefix(pkg).replace("-", " ").title())} by {_bold(info["author"])}

{info["description"]}
{
    NL+_bold("KJSPKG Lookup")+": https://kjspkglookup.modernmodpacks.site/#"+pkg+NL if ":" not in pkg or pkg.split(":")[0]=="kjspkg" else ""
}"""+(f"""{_bold("Downloads")}: {info['lookupapi']['downloads']}
{_bold("Views")}: {info['lookupapi']['views']}
""" if "lookupapi" in info.keys() else "\n")+f"""
{_bold("Dependencies")}: {", ".join([_remove_prefix(i).title().replace("-", " ").replace("_", " ")+(" ("+i.split(":")[0].title()+")" if ":" in i else "") for i in info["dependencies"]]) if "dependencies" in info.keys() and len(info["dependencies"])>0 else "*nothing here*"}
{_bold("Incompatibilities")}: {", ".join([_remove_prefix(i).title().replace("-", " ").replace("_", " ")+(" ("+i.split(":")[0].title()+")" if ":" in i else "") for i in info["incompatibilities"]]) if "incompatibilities" in info.keys() and len(info["incompatibilities"])>0 else "*compatible with everything!*"}

{_bold("Versions")}: {", ".join([f"1.{10+i}" for i in info["versions"]])}
{_bold("Modloaders")}: {", ".join([i.title() for i in info["modloaders"]])}

{_bold("GitHub")}: https://github.com/{info["repo"]}/tree/{info["branch"]}"""+
(f"""
{_bold('License')}: {info['ghdata']['license']['key'].upper() if info['ghdata']['license']!=None else 'ARR'}

{_textbg(f" 👁️  {info['ghdata']['watchers_count']} ")} {_textbg(f" 🍴 {info['ghdata']['forks_count']} ")} {_textbg(f" ⭐ {info['ghdata']['stargazers_count']} ")}
""" if "ghdata" in info else "\n"))
def fetch(*, logo:bool=True, script:bool=False): # Fetch data about the project in a pfetch-esque format
    versions = _get_versions() # Get mod versions
    data = { # Compile all required data
        "version": f"1.{10+kjspkgfile['version']}",
        "loader": kjspkgfile["modloader"],
        "pkgs": len(kjspkgfile["installed"].keys()),
        "kube": versions["kubejs"] if "kubejs" in versions.keys() else "???",
        "rhino": versions["rhino"] if "rhino" in versions.keys() else "???",
        "arch": versions["architectury"] if "architectury" in versions.keys() else "???"
    }

    # Print it (scripty)
    if script:
        print(dumps(data))
        return

    # Prepare it to look pretty
    datastr = _bold(f"KJSPKG@{getcwd()}\n")
    longeststr = len(max(data.keys(), key=len))+1
    for k,v in data.items(): datastr += f"{_purple(k)}{' '*(longeststr-len(k))}{v}\n" 
    selectedlogo = LOGO if logo else ""

    # Print it (pretty)
    for l, d in zip_longest(selectedlogo.splitlines()[1:], datastr.splitlines()): print(f"{_purple(l)+'    ' if l!=None else ''}{d if d!=None else ''}")
    print()
def listall(*, count:bool=False, search:str="", reload:bool=True): # List all pkgs
    if reload: _reload_pkgs() # Reload pkgs
    allpkgs = list(_pkgs_json().keys()) # All package names

    if count: # If count is true
        print(len(allpkgs)) # Print the pkg count
        return
    if not search: # If no search query
        print("\n".join(sorted(allpkgs))) # Print all pkg names
        return

    filterwarnings("ignore") # Ignore the warning thefuzz produces
    from thefuzz import process # Fuzzy search

    # Get results and print the best ones
    results = process.extract(search, allpkgs, limit=25)
    for result, ratio in results:
        if ratio>75: print(result)
def search(*query:str, **kwags): # Search for pkgs
    listall(search="-".join(query), **kwags) # Call listall with joined spaces
def reload(): _reload_pkgs() # Reload packages
def init(*, quiet:bool=False, override:bool=False, cancreate:str=None, **configargs): # Init project
    global kjspkgfile

    params = [ # Changable parameters
        "version",
        "modloader",
        "trustgithub"
    ]
    for arg in configargs.keys(): # If a parameter is not found, raise key err
        if arg not in params: raise TypeError()

    if cancreate: # Scriptable cancreate option
        print(_check_project())
        return

    if not _check_project(): _err("Hmm... This directory doesn't look like a kubejs directory") # Wrong dir err

    if _project_exists() and not override: # Override
        if not quiet and input("\u001b[31;1mA PROJECT ALREADY EXISTS IN THIS REPOSITORY, CREATING A NEW ONE OVERRIDES THE PREVIOUS ONE, ARE YOU SURE YOU WANT TO PROCEED? (y/N): \u001b[0m").lower()=="y" or override: _delete_project()
        else: exit(0)

    # Ask for missing params
    if "version" not in configargs.keys(): configargs["version"] = input(_bold("Input your minecraft version (1.12/1.16/1.18/1.19/1.20/1.21): ")) # Version
    if configargs["version"] not in VERSIONS.keys(): _err("Unknown or unsupported version: "+str(configargs["version"]))
    configargs["version"] = VERSIONS[configargs["version"]]

    if "modloader" not in configargs.keys(): configargs["modloader"] = input(_bold("Input your modloader (forge/neoforge/fabric/quilt): ")) # Modloader
    configargs["modloader"] = configargs["modloader"].lower()
    if configargs["modloader"] not in ("forge", "neoforge", "fabric", "quilt"): _err("Unknown or unsupported modloader: "+configargs["modloader"].title())

    if configargs["modloader"] in ("forge", "neoforge"): configargs["modloader"] = "forge"
    elif configargs["modloader"] in ("fabric", "quilt"): configargs["modloader"] = "fabric"

    _create_project_directories() # Create .kjspkg directories
    if configargs["version"]==6: _enable_reflection() # Enable reflection in the config for 1.16.5

    kjspkgfile = CONFIG # Set .kjspkg to default config
    for k, v in configargs.items(): kjspkgfile[k] = v # Change config as needed

    with open(".kjspkg", "w+") as f: dump(kjspkgfile, f) # Create .kjspkg file
    if not quiet: print(_bold("Project created!")) # Woo!
def uninit(*, confirm:bool=False): # Remove the project
    if confirm or input("\u001b[31;1mDOING THIS WILL REMOVE ALL PACKAGES AND UNINSTALL KJSPKG COMPLETELY, ARE YOU SURE YOU WANT TO PROCEED? (y/N): \u001b[0m").lower()=="y": 
        _delete_project()
        print("\u001b[31;1mProject deleted\u001b[0m")
    else: print(_bold("Aborted."))

# DEV COMMAND FUNCTIONS
def devrun(launcher:str=None, version:int=None, modloader:str=None, ignoremoddeps:bool=False, quiet:bool=False): # Run a test instance
    global kjspkgfile

    # Errs
    if not path.exists(".kjspkg"): _err(".kjspkg file not found. Add one according to the KJSPKG README (https://github.com/Modern-Modpacks/kjspkg/blob/main/README.md) and then re-run.") # No .kjspkg file error
    manifest = load(open(".kjspkg")) # Parse the json file
    if "description" not in manifest.keys(): _err("Invalid manifest. You are probably running this command inside of an instance instead of a package.") # Invalid schema error

    if version==None: # If the version is not specified
        if len(manifest["versions"])==1: version = str(manifest["versions"][0]) # Select the only one
        else: version = input(_bold(f"What version would you like to test the package for? ({'/'.join([str(i) for i in manifest['versions']])}) ")).lower() # Or ask if multiple
    if not (isinstance(version, int) or version.isnumeric()) or int(version) not in manifest['versions']: _err("Unknown version: "+version) # Err if the version is unknown
    if version=="2": _err("Testing for 1.12 is not supported") # 1.12 not supported :shrug:

    if modloader==None: # If the modloader is not specified
        if len(manifest["modloaders"])==1: modloader = manifest["modloaders"][0] # Select the only one
        else: modloader = input(_bold(f"What modloader would you like to test the package for? ({'/'.join(manifest['modloaders'])}) ")).lower() # Or ask if multiple
    if modloader not in manifest['modloaders']: _err("Unknown modloader: "+modloader) # Err if the modloader is unknown

    if osname=="nt": # Windows paths
        LAUNCHERPATHS = {
            "PrismLauncher": path.join(getenv("LOCALAPPDATA"), "Programs", "PrismLauncher", "prismlauncher.exe"),
            "multimc": path.join(getenv("USERPROFILE"), "Downloads", "MultiMC", "MultiMC.exe")
        }
    else: # Posix paths
        LAUNCHERPATHS = {
            "PrismLauncher": "/usr/bin/prismlauncher",
            "multimc": "/opt/multimc/run.sh"
        }

    if launcher==None: # If the launcher is not specified
        launcher = ""
        for l, p in LAUNCHERPATHS.items():
            if path.exists(p):
                launcher = l # Find the one that's installed
                break
        if not launcher: _err("Prism/MultiMC was not found. Please install one of these launchers.")
    else: # If it is specified
        if launcher.lower() in ("prism", "prismlauncher"): launcher = "PrismLauncher" # Replace prism with prismlauncher
        if launcher.lower()=="multi": launcher = "multimc" # Replace multi with multimc

        if launcher.lower() not in [i.lower() for i in LAUNCHERPATHS.keys()]: _err("Launcher doesn't exist or not supported: "+launcher.title()) # Err if unknown launcher
        elif not path.exists({k.lower(): v for k, v in LAUNCHERPATHS.items()}[launcher.lower()]): _err("Launcher not installed: "+launcher.title()) # Err if the launcher isn't installed

    # Kill all launcher windows
    for i in process_iter():
        if launcher.lower() in i.name().lower(): i.kill()

    instancename = f"kjspkg{version}{manifest['modloaders'][0]}" # Instance name
    if osname=="posix": instancepath = path.expanduser(f"~/.local/share/{launcher}/instances/{instancename}") # Linux instance path
    else:
        if launcher=="multimc": instancepath = path.join(path.sep.join(LAUNCHERPATHS[launcher].split(path.sep)[:-1]), "instances", instancename) # Windows multimc instance path
        elif launcher=="PrismLauncher": instancepath = path.join(getenv("APPDATA"), "PrismLauncher", "instances", instancename) # Windows prism instance path
    instancezippath = path.join(gettempdir(), instancename+".zip") # Path to instance's zipped file
    if not path.exists(instancepath): # If instance doesn't exit
        with open(instancezippath, "wb+") as f:
            f.write(get(f"https://github.com/Modern-Modpacks/kjspkg/raw/main/instances/{instancename}.zip").content) # Downlaod the zip

        if not quiet: print(_bold("A launcher window should now appear, please import the instance, config it if you need and close the window."))
        run([LAUNCHERPATHS[launcher], "-I", instancezippath], stdout=DEVNULL, stderr=DEVNULL) # Import

    pkgpath = getcwd() # Save the package file
    chdir(path.join(instancepath, ".minecraft", "kubejs")) # Change the cwd to the instance
    kjspkgfile = load(open(".kjspkg")) # Load the .kjspkg file

    if path.exists("tmp"): rmtree("tmp", onerror=_dumbass_windows_path_error) # Remove the temp folder if exists

    # Install deps
    if "dependencies" in manifest.keys():
        modids = _get_modids() # Get mod ids
        for dep in manifest["dependencies"]: # For each dep
            if dep.startswith("mod:") and _remove_prefix(dep.lower()) not in modids: # If a mod dep is not found
                if ignoremoddeps and not quiet: print(_bold(f"!!! Mod `{_remove_prefix(dep)}` is not installed, but it's ignored since ignoremoddeps is specified.")) # Warn if ignoremoddeps is true
                else: _err(f"Your package depends on `{_remove_prefix(dep)}`, which is not installed.") # Or err if it's false
            elif not dep.startswith("mod:"): _install_pkg(dep, False, True, True, True) # Install package deps

    makedirs("tmp", exist_ok=True) # Create a tempdir
    tmpdir = copytree(pkgpath, "tmp/test") # Copy the test package contents
    _move_pkg_contents("test", tmpdir, ".") # Install
    rmtree(tmpdir, onerror=_dumbass_windows_path_error) # Remove the temp folder

    if not quiet: loadthread = _loading_thread("Running test instance...") # Loading anim
    try: run([LAUNCHERPATHS[launcher], "-l", instancename], stdout=DEVNULL, stderr=DEVNULL) # Run the instance
    except KeyboardInterrupt: pass # Stop on keyboard interrupt

    if not quiet:
        loadthread.terminate() # Terminate loading anim
        loadthread.join() # Wait for it to finish

    _remove_pkg("test", True) # Remove the test package
    for dep in manifest["dependencies"]:
        if not dep.startswith("mod:"): _remove_pkg(dep, True) # Remove all deps

    chdir(pkgpath) # Change the cwd back to the package dir
    if not quiet: print(_bold("Test instance killed ✓")) # 👍
def devdist(description:str=None, author:str=None, dependencies:list=None, incompatibilities:list=None, versions:list=None, modloaders:list=None, distdir:str="dist", gitrepository:bool=True, generatemanifest:bool=True, quiet:bool=False): # Package the scripts automatically
    if not _check_project(): _err("Current directory does not look like a KubeJS one...") # Check if the cwd is a kubejs one

    # Inputs
    if generatemanifest:
        if description==None: description = input(_bold("Input a description for your package: "))
        if author==None: author = input(_bold("Enter authors' names that worked on the package")+" (comma separated): ")
        if dependencies==None: dependencies = input(_bold("Enter dependency names for your package")+" (comma separated, optional): ").lower().replace(" ", "").split(",")
        if dependencies==[""]: dependencies=[] # Set the deps to empty if the input is empty
        if incompatibilities==None: incompatibilities = input(_bold("Enter incompatibility names for your package")+" (comma separated, optional): ").lower().replace(" ", "").split(",")
        if dependencies==[""]: dependencies=[] # Set the incompats to empty if the input is empty

        kjspkgfile = None
        if (versions==None or modloaders==None) and path.exists(".kjspkg"): kjspkgfile = load(open(".kjspkg")) # Load .kjspkg if exists

        if kjspkgfile!=None: # Extract the version number and modloader from .kjspkg
            if versions==None: versions = (kjspkgfile["version"],)
            if modloaders==None: modloaders = (kjspkgfile["modloader"],)
        else: # If .kjspkg is not found
            if versions==None:
                versions = input(_bold("Enter the version keys for your package")+" (6/8/9/10, comma separated): ").replace(" ", "").split(",") # Ask for version
                for i in versions:
                    if not i.isnumeric or int(i) not in VERSIONS.values(): _err("Unknown version: "+i)
                versions = [int(i) for i in versions]
            if modloaders==None:
                modloaders = input(_bold("Enter the modloaders for your package")+" (forge/fabric, comma separated): ").replace(" ", "").lower().split(",") # Ask for modloader
                for i in modloaders:
                    if i not in ("forge", "fabric"): _err("Unknown modloader: "+i)

    # Confirmations
    if not quiet: input(_bold("Only scripts and assets that start with `kjspkg_` will be added (prefix removed). Press enter to confirm "))
    if path.exists(distdir):
        if input(_bold(f"The `{distdir}` folder already exists, remove it?")+" (Y/n): ").lower()!="n" or quiet: rmtree(distdir, onerror=_dumbass_windows_path_error)
        else: _err("Aborted.")

    for dir in SCRIPT_DIRS+ASSET_DIRS: # For all script and asset dirs
        makedirs(path.join(distdir, dir), exist_ok=True) # Make dirs in dist
        for dirpath, _, files in walk(dir): # For all files
            for name in files:
                if name.startswith("kjspkg_"): # If the file starts with kjspkg_
                    makedirs(path.join(distdir, dirpath), exist_ok=True) # Create parents
                    copy(path.join(dirpath, name), path.join(distdir, dirpath, name.removeprefix("kjspkg_"))) # Copy it
    
    # Write .kjspkg manifest
    if generatemanifest:
        with open(path.join(distdir, ".kjspkg"), "w+") as f:
            dump({
                "author": author,
                "description": description,
                
                "versions": versions,
                "modloaders": modloaders,
                "dependencies": dependencies,
                "incompatibilities": incompatibilities
            }, f)

    if not quiet: print(_bold("Package generated ✓")) # Woo

    # Set up git repo
    if gitrepository:
        repo = Repo.init(distdir) # Init
        repo.git.add(all=True) # Add
        repo.index.commit("Initial Commit") # Commit
        if not quiet: print(_bold("Git repository initialized (don't forget to add a license) ✓")) # Cool
def devtest(legacychecks:bool=True): # Test scripts for errors
    if not path.exists(".kjspkg"): _err(".kjspkg not found") # Check for .kjspkg
    manifest = load(open(".kjspkg")) # Load it
    errors = False # No errors at first

    for dir in SCRIPT_DIRS: # For each script dir
        for dirpath, _, files in walk(dir): # For each nested file
            for name in files:
                if not name.endswith(".js"): continue # If the file is not a js file, skip
                script = path.join(dirpath, name) # Get the path to the script

                try: parsedscript = parse(open(script).read()).toDict() # Read it and parse it
                except error_handler.Error as e: # If any errors come up during parsing
                    errors = True # Set the errs to true
                    _err(f"[{script}] {e.message}", True) # Show the error
                    continue # Skip to the next script

                for k, v in flatten(parsedscript).items(): # For each key in flattened tree
                    if ("version" in manifest.keys() and manifest["version"]==9) or ("versions" in manifest.keys() and any([i>=9 for i in manifest["versions"]])): # If the version is 1.19+
                        if legacychecks and k.endswith("_callee_name") and v in ("onEvent", "java"): # And the script uses onEvent/java (+legacy checks are on)
                            errors = True # Set the errs to true
                            _err(f"[{script}] You are using the {v} method, but your script is marked as KJS6+. Please use the compat layer (https://kjspkglookup.modernmodpacks.site/#kjspkg-compat-layer) or change your script according to the migration guide (https://wiki.latvian.dev/books/kubejs-legacy/page/migrating-to-kubejs-6).", True) # Show the legacy error
                    else: # If the version is 1.18/1.16
                        if legacychecks and k.endswith("_callee_object_name") and v in ( # And the script uses KJS6 event syntax (+legacy checks are on)
                            "StartupEvents",
                            "ServerEvents",
                            "ClientEvents",
                            "LevelEvents",
                            "PlayerEvents",
                            "EntityEvents",
                            "BlockEvents",
                            "ItemEvents",
                            "NetworkEvents",
                            "JEIEvents",
                            "REIEvents"
                        ):
                            errors = True # Set the errs to true
                            _err(f"[{script}] You are using the {v} class, but your script is marked as KJS legacy. Please use the compat layer (https://kjspkglookup.modernmodpacks.site/#kjspkg-compat-layer) or change your script according to the migration guide (https://wiki.latvian.dev/books/kubejs-legacy/page/migrating-to-kubejs-6).", True) # Show the legacy error

    if not errors: print(_bold("No errors found ✓")) # If no errors were found, display this message

# INFO COMMAND FUNCTIONS
def info(): # Print the help page
    SPLASHES = [ # Splash list
        "You should run `kjspkg uninit`, NOW!",
        "Run `kjspkg mold` to brew kombucha",
        "Thanks Lat 👍",
        "Help, I'm locked in a basement packaging scripts!",
        "kjspkg rm -rf / --no-preserve-root",
        "Made in Python 3.whatever!",
        "https://modernmodpacks.site",
        "Made by Modern Modpacks!",
        "gimme gimme gimme",
        "`amogus` is a real package!",
        "Supports 1.12!",
        "Procrastinating doing one project by doing another project, genius!",
        "Also try Magna!",
        "No alternative for CraftTweaker!"
    ]

    # Info string
    INFO = f"""
{_bold("KJSPKG")}, a package manager for KubeJS.
{choice(SPLASHES)+NL if _check_for_fun() else ""}
{_bold("Commands:")}

kjspkg install/download [pkgname1] [pkgname2] [--quiet/--skipmissing] [--update] [--noreload] - installs packages
kjspkg remove/uninstall [pkgname1] [pkgname2] [--quiet/--skipmissing] - removes packages
kjspkg update [pkgname1/*] [pkgname2] [--quiet/--skipmissing] - updates packages
kjspkg updateall [--quiet/--skipmissing] - updates all packages

kjspkg install [pkgname] - installs packages from kjspkg's repo
kjspkg install kjspkg:[pkgname] - installs packages from kjspkg's repo
kjspkg install github:[author]/[name] - installs external packages from github

kjspkg list [--count] - lists packages (or outputs the count of them)
kjspkg pkg [package] [--script] [--nogithubinfo] - shows info about the package
kjspkg fetch [--nologo] [--script] - prints out a neofetch-eqsue screen with info about the current KJSPKG instance
kjspkg listall/all [--count] [--search "<query>"] [--noreload] - lists all packages
kjspkg search [query] - searches for packages with a similar name
kjspkg reload/refresh - reloads the cached package registry

kjspkg init [--override/--quiet] [--version "<version>"] [--modloader "<modloader>"] [--cancreate "<path>"] - inits a new project (will be run by default)
kjspkg uninit [--confirm] - removes all packages and the project

kjspkg help/info - shows this message (default behavior)
kjspkg dev - shows info about dev commands
kjspkg gui - shows info about the GUI app

{_bold("Credits:")}

Modern Modpacks - Owner
G_cat101 - Coder
Tizu69 - Maintainer of KJSPKG Lookup
Juh9870 - Wanted to be here
    """

    print(INFO) # Print the info
def devinfo(): # Print info about the dev commands
    # Info string
    INFO = f"""
{_bold("Dev commands")}
Dev utils are experimental, use at your own risk.

kjspkg dev run [--quiet] [--ignoremoddeps] [--launcher "<launcher>"] [--version "<version>"] [--modloader "<modloader>"] - runs your package in a test minecraft instance (requires MultiMC/Prism to be installed)
kjspkg dev dist [--quiet] [--nogitrepository] [--nogeneratemanifest] [--description "<description>"] [--author "<author>"] [--dependencies "<dep1>,<dep2>"] [--incompatibilities "<incompat1>,<incompat2>"] [--versions "<version1>,<version2>"] [--modloaders "<forge>,<fabric>"] [--distdir "<directory>"] - creates a package from your kubejs folder
kjspkg dev test [--nolegacychecks] - checks your code for syntax errors

kjspkg dev help - shows this message (default behavior)
    """

    print(INFO) # Print the info
def guiinfo(): # Print info about the GUI app
    print(f"{_bold('Did you know there is a GUI app for KJSPKG?')} Check it out at https://github.com/Modern-Modpacks/kjspkg-gui!")
def kombucha(): # Kombucha easter egg
    RECIPE = f"""
{_bold("Ingredients")}

* 2 organic green teabags (or 2 tsp loose leaf)
* 2 organic black teabags bags (or 2 tsp loose leaf)
* 100-200g granulated sugar, to taste
* 1 medium scoby, plus 100-200ml starter liquid

{_bold("Method")}

STEP 1
For essential information on brewing safely, our top recipe tips and fun flavours to try, read our guide on how to make kombucha. Pour 1.8 litres boiled water into a saucepan, add the teabags and sugar (depending on how sweet you like it or the bitterness of your tea), stir to dissolve the sugar and leave for 6-10 mins to infuse.

STEP 2
Remove and discard the teabags without squeezing them. Leave the tea to cool completely before pouring into a large 2.5- to 3-litre glass jar. Add the scoby and its starter liquid, leaving a minimum of 5cm space at the top of the jar.

STEP 3
Cover the jar with a thin tea towel or muslin cloth so the scoby can 'breathe'. Secure with an elastic band and label the jar with the date and its contents.

STEP 4
Leave to ferment for one to two weeks at room temperature and away from radiators, the oven or direct sunlight. Do not put the jar in a cupboard, as air circulation is important.

STEP 5
After the first week, taste the kombucha daily – the longer you leave it, the more acidic the flavour will become. When ready, pour the kombucha into bottles, making sure to reserve the scoby and 100-200ml of starter fluid for the next batch.

STEP 6
The kombucha is ready to drink immediately, or you can start a ‘secondary fermentation’ by adding flavours such as fruit, herbs and spices to the drawn-off liquid and leaving it bottled for a few more days before drinking. Will keep in the fridge for up to three months.
    """
    print(RECIPE)
 
# PARSER FUNCTION
def _parser(func:str="help", *args, help:bool=False, **kwargs):
    global kjspkgfile

    system("") # Enable color codes on windows, don't ask me why

    if help: func="help"

    devparser = False
    if func=="dev":
        devparser = True # If the func is equal to dev, set the parser to the dev parser
        if args: # If the args aren't empty
            func = args[0] # Set the func to the next argument
            args = args[1:] # And move the args by 1
        else: func="help" # If the args are empty, default to dev help

    if not devparser:
        FUNCTIONS = { # Non-dev command mappings
            "install": install,
            "download": install,
            "add": install,
            "remove": removepkg,
            "uninstall": removepkg,
            "rm": removepkg,
            "update": update,
            "updateall": updateall,
            "list": listpkgs,
            "pkg": pkginfo,
            "pkginfo": pkginfo,
            "fetch": fetch,
            "listall": listall,
            "all": listall,
            "search": search,
            "reload": reload,
            "refresh": reload,
            "init": init,
            "uninit": uninit,
            "help": info,
            "info": info,
            "gui": guiinfo
        }
        if _check_for_fun(): FUNCTIONS["mold"] = kombucha
    else:
        FUNCTIONS = { # Dev command mappings
            "run": devrun,
            "dist": devdist,
            "package": devdist,
            "test": devtest,
            "check": devtest,
            "validate": devtest,
            "help": devinfo,
            "info": devinfo
        }

    if func not in FUNCTIONS.keys(): _err(f"Command \"{func}\" is not found. Run \"kjspkg {'dev ' if devparser else ''}help\" to see all of the available commands") # Wrong command err
    
    if not devparser: # Skip the .kjspkg file stuff if the parser is the dev parser
        helperfuncs = (info, guiinfo, init, pkginfo, listall, search, kombucha) # Helper commands that don't require a project
        if FUNCTIONS[func] not in helperfuncs and not _project_exists(): # If a project is not found, call init
            print(_bold("Project not found, a new one will be created.\n"))
            init()
        if (FUNCTIONS[func]==init or FUNCTIONS[func] not in helperfuncs) and path.exists(".kjspkg"): kjspkgfile = load(open(".kjspkg")) # Open .kjspkg

    FUNCTIONS[func](*args, **kwargs) # Run the command

    # Clean up
    if not devparser and path.exists(".kjspkg") and FUNCTIONS[func] not in helperfuncs: # If uninit wasn't called, the command isn't a helper command and the parser is not dev
        _update_manifest() # Update .kjspkg
        with open(".kjspkg", "w") as f: dump(kjspkgfile, f) # Save .kjspkg

# RUN
if __name__=="__main__": # If not imported
    _clear_tmp() # Remove tmp

    try: Fire(_parser) # Run parser with fire
    except (KeyboardInterrupt, EOFError): exit(0) # Ignore some exceptions
    # except TypeError: _err("Wrong syntax") # Wrong syntax err
    except GitCommandNotFound: _err("Git not found. Install it here: https://git-scm.com/downloads") # Git not found err
    except (exceptions.ConnectionError, exceptions.ReadTimeout): _err("Low internet connection") # Low internet connection err

    _clear_tmp() # Remove tmp again

# Ok that's it bye