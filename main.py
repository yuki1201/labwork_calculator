import tkinter as tk
from cffi import FFI

ffi = FFI()

rustlib = ffi.dlopen('lib/target/release/libcalcr.dylib')

ffi.cdef("""
    float rust_fn(const char *n);
""")

BUTTONS = [
  ['MC','MR','M+','M-','GT'],
  ['tax','(', ')', '^','%'],
  ['CE','7', '8', '9', '/'],
  ['C', '4', '5', '6', '*'],
  ['AC','1', '2', '3', '-'],
  ['0', '00','.', '=','+']
]

mem=0.0
gt=0.0
def make_click(ch):
    def click(e):
        global mem
        global gt
        if ch == '=': 
          label["text"] = '= ' + str(calc())
          gt=gt+float(calc())
          return
        elif ch == 'tax': 
          disp.insert(tk.END, '+tax')
          return
        elif ch == 'CE': 
          disp.delete(len(disp.get())-1,tk.END)
          return
        elif ch == 'C': 
          disp.delete(0,tk.END)
          label["text"] =''
          return
        elif ch == 'AC': 
          disp.delete(0,tk.END)
          label["text"] =''
          mem=0.0
          gt=0.0
          return
        elif ch == 'M+': 
          mem=mem+float(calc())
          label["text"] = '= ' + str(calc())
          return
        elif ch == 'M-': 
          mem=mem-float(calc())
          label["text"] = '= ' + str(calc())
          return
        elif ch == 'MR': 
          disp.insert(tk.END, mem)
          return
        elif ch == 'MC': 
          mem=0.0
          return
        elif ch == 'GT': 
          disp.insert(tk.END, gt)
          return
        else: 
          disp.insert(tk.END, ch)
    return click


def calc():
    myname = disp.get()
    myname=myname.replace('+tax', '*1.1')
    myname=myname.replace('%', '*0.01')
    name = ffi.new('char[]', bytes(myname, 'utf-8'))
    msg = rustlib.rust_fn(name)
    return float(format(msg))


win = tk.Tk()
win.title("万能でんたくんα")
win.geometry("320x270")
win.configure(background='#3E4149')
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

win.mainloop()