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
        [sg.Text('Welcome To Dockerfreeze GUI')],
        [sg.T(" " * 7), sg.Text('Gitpod Ready', size=(16, 1)), sg.Checkbox("", default=False)],
        [sg.T(" " * 7), sg.Text('Don\'t Add Programs', size=(16, 1)), sg.Checkbox("", default=False)],
        [sg.T(" " * 3), sg.Submit("BUILD"), sg.Cancel()]
    ]

    mainWindow = sg.Window('Dockerfreeze GUI', default_element_size=(40, 1)).Layout(layout)
    cprint("Main Window Running", "green")

    args = []
    
    while True:  # Event Loop
        button, values = mainWindow.Read()
        if button in ("Cancel", None):
            break
        if button == "BUILD":
            did_error = False
            command = ""
            if True in values:
                command = "dockerfreeze -"
                if values[0]:
                    command += "g"
                if values[1]:
                    command += "n"
                command += " &> /dev/null"
            else:
                command = "dockerfreeze &> /dev/null"
            cprint(command, "blue")
            if exists("Dockerfile"):
                warn("File already exists")
                continue
            else:
                if system(command) != 0:
                    did_error = True
            if did_error:
                warn("File already exists")
            else:
                sg.popup("Dockerfile Built")
            did_error = False
    cprint("Closing Application...", "red")
    mainWindow.close()


if __name__ == "__main__":
    main()
