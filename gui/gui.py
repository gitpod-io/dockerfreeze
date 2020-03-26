import PySimpleGUI as sg
from os import system
from os.path import exists
from termcolor import cprint

# Theme
sg.theme('DarkAmber')


def warn(error):
    cprint(f"Error: {error}", "red")
    sg.popup(f"Error: {error}", title="Attention!")


def main():
    layout = [
        [sg.Text('Welcome To DockerFreeze GUI')],
        [sg.T(" " * 7), sg.Text('Gitpod Ready', size=(12, 1)), sg.Checkbox("", default=False)],
        [sg.T(" " * 3), sg.Submit("BUILD"), sg.Cancel()]
    ]

    mainWindow = sg.Window('Docker Freeze Gui', default_element_size=(40, 1)).Layout(layout)
    cprint("Main Window Running", "green")
    
    while True:  # Event Loop
        button, values = mainWindow.Read()
        if button in ("Cancel", None):
            break
        if button == "BUILD":
            did_error = False
            if exists("Dockerfile"):
                warn("File already exists")
                continue
            if values[0]:
                if system("dockerfreeze -g &> /dev/null") != 0:
                    did_error = True
            else:
                if system("dockerfreeze &> /dev/null") != 0:
                    did_error = True
            if did_error:
                warn("File already exists")
            else:
                sg.popup("Dockerfile Built")
            did_error = False
    cprint("Closing Application...", "red")
    mainWindow.close()


main()
