# APPL Installer
# Creates a config folder at ~/.config/appl/
# Clones git repo to location of choice
# Allows selection of package database templates (called trees)
import pwd, os
from simple_term_menu import TerminalMenu

def clear():
    os.system('cls' if os.name == 'nt' else 'clear')
clear()

USERNAME = pwd.getpwuid(os.getuid())[0]
PATH = "/home/"+USERNAME+"/.config/appl/"
print(f"Checking to see if {USERNAME}'s config folder exists")
if not os.path.exists(PATH):
    
    os.mkdir(PATH)
    print(f"Config folder doesn't exist, created at {PATH}")

else:
    print("Config folder already exists")

print("------------------------")
print("     APPL Installer     ")
print("------------------------\n")
print("Select an option to customize installation \n\n\n")

Options = ["Proceed with installation", "Modify install location", "Exit"]
menu = TerminalMenu(Options)
choice = menu.show()



