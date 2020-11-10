import tkinter as tk
from cffi import FFI

ffi = FFI()

rustlib = ffi.dlopen('lib/target/release/libcalcr.dylib')

ffi.cdef("""
    float rust_fn(const char *n);
""")

BUTTONS = [
  ['(', ')', '^', '='],
  ['7', '8', '9', '/'],
  ['4', '5', '6', '*'],
  ['1', '2', '3', '-'],
  ['0', '.', 'C', '+']
]


def make_click(ch):
    def click(e):
        #print(ch)
        if ch == '=': 
          calc(0)
          return
        elif ch == 'C': 
          disp.delete(len(disp.get())-1,tk.END)
          return
        else: disp.insert(tk.END, ch)
    return click


def calc(e):
    myname = disp.get()
    name = ffi.new('char[]', bytes(myname, 'utf-8'))
    msg = rustlib.rust_fn(name)
    label["text"] = '= ' + str(format(msg))


win = tk.Tk()
win.title("万能でんたくんα")
win.geometry("255x240")
disp = tk.Entry(win, font=('', 20), justify="center",highlightbackground='#3E4149')
disp.pack(fill='x')
disp.bind('<Return>', calc)
label = tk.Label(win, font=('', 20), anchor="center",highlightbackground='#3E4149')
label.pack(fill='x')
fr = tk.Frame(win)
fr.pack()
for y, cols in enumerate(BUTTONS):
    for x, n in enumerate(cols):
        btn = tk.Button(fr,text=n,font=('', 20),width=3,height=1,highlightbackground='#3E4149')
        btn.grid(row=y+1, column=x+1)
        btn.bind('<1>', make_click(n))


# ウィンドウを動かす --- (*7)

win.mainloop()