import PySimpleGUI as sg
from os import system
from os.path import exists

# Theme
sg.theme('DarkAmber')

layout = [
    [sg.Text('Welcome to the DockerFreeze GUI, written in Python.')],
    # [sg.Text('Add Environment Variables', size=(23, 1)), sg.Checkbox("", default=False)],
    # [sg.Text('     Pretty Print Dockerfile', size=(23, 1)), sg.Checkbox("", default=False)],
    # [sg.Text('      Install user packages', size=(23, 1)), sg.Checkbox("", default=False)],
    [sg.Text('           Gitpod Ready', size=(23, 1)),
     sg.Checkbox("", default=False)],
    # [sg.Text('              Vagrant', size=(23, 1)), sg.Checkbox("", default=False)],
    [sg.Submit("BUILD"), sg.Cancel()]]


window = sg.Window('Docker Freeze Gui',
                   default_element_size=(40, 1)).Layout(layout)
while True:
    button, values = window.Read()
    if button in (None, "Cancel"):
        break
    elif button == "BUILD":
        did_error = False
        if exists("Dockerfile"):
            sg.popup("Error: File already exists")
            continue
        if values[0]:
            if system("dockerfreeze -g &> /dev/null") != 0:
                did_error = True
        else:
            if system("dockerfreeze &> /dev/null") != 0:
                did_error = True
        if did_error:
            sg.popup("Error: File already exists")
        else:
            sg.popup("Dockerfile Built")
        did_error = False

if __name__ == '__main__':
    print("Starting...")
    print(button, values[0:])
