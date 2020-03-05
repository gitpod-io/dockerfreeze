import sys
import PySimpleGUI as sg

# Import tkinter, required for PySimpleGUI
if sys.version_info[0] == 3:
    import tkinter as tk
    from tkinter import *
elif sys.version_info[0] == 2:
    import tkinter as Tk
    from Tkinter import *
else:
    sys.exit(255)

# Theme
sg.theme('DarkAmber')

layout = [
            [sg.Text('Welcome To Docker Freeze Gui, written in python.')],
            [sg.Text('Add Environment Variables', size=(23, 1)), sg.Checkbox("", default=False)],
            [sg.Text('     Pretty Print Dockerfile', size=(23, 1)), sg.Checkbox("", default=False)],
            [sg.Text('      Install user packages', size=(23, 1)), sg.Checkbox("", default=False)],
            [sg.Text('           Gitpod Ready', size=(23, 1)), sg.Checkbox("", default=False)],
            [sg.Text('              Vagrant', size=(23, 1)), sg.Checkbox("", default=False)],
            [sg.Submit("BUILD"), sg.Cancel()]]


window = sg.Window('Docker Freeze Gui', default_element_size=(40, 1)).Layout(layout)
button, values = window.Read()

if __name__ == '__main__':
    print(button, values[0:])
