import tkinter as tk
from cffi import FFI

ffi = FFI()
rustlib = ffi.dlopen('lib/target/release/libcalcr.dylib')
ffi.cdef("""float rust_fn(const char *n,bool);""")

BUTTONS = [
  ['rpn','in','spc'],
  ['MC','MR','M+','M-','GT'],
  ['+tax','(', ')', '^','%'],
  ['CE','7', '8', '9', '/'],
  ['C', '4', '5', '6', '×'],
  ['AC','1', '2', '3', '-'],
  ['0', '00','.', '=','+']
]

mode=True
mem=0.0
gt=0.0
def make_click(ch):
    def click(e):
        global mem
        global gt
        global mode
        if ch == '=': 
          ans=round(calc(),4)
          label["text"] = '= ' + str(ans)
          gt=gt+ans
          return
        elif ch == 'in':
          mode=True
          return
        elif ch == 'rpn':
          mode=False
          return
        elif ch =='spc':
          disp.insert(tk.END, ' ')
          return
        elif ch == 'MC': 
          mem=0.0
          return
        elif 'C' in ch:
          if ch == 'CE': 
            disp.delete(len(disp.get())-1,tk.END)
          else:
            disp.delete(0,tk.END)
            label["text"] =''
            if ch == 'AC':
              mem=0.0
              gt=0.0
          return 
        elif ch == 'M+' or ch == 'M-': 
          ans=round(calc(),4)
          if ch == 'M+':
            mem=mem+ans
          else:
            mem=mem-ans
          label["text"] = '= ' + str(ans)
          return
        elif ch == 'MR': 
          disp.insert(tk.END, mem)
          return
        elif ch == 'GT': 
          disp.insert(tk.END, gt)
          return
        elif ch=='×':
          disp.insert(tk.END,'*')
        else: 
          disp.insert(tk.END, ch)
    return click


def calc():
    myname = disp.get()
    myname=myname.replace('+tax', '*1.1')
    myname=myname.replace('%', '/100')
    name = ffi.new('char[]', bytes(myname, 'utf-8'))
    msg = rustlib.rust_fn(name,mode)
    return float(format(msg))


win = tk.Tk()
win.title("万能でんたくんβ")
win.geometry("300x270")
win.configure(background='#3E4149')
disp = tk.Entry(win, font=('', 20), justify="center",highlightbackground='#3E4149')
disp.pack(fill='x')
disp.bind('<Return>', calc)
label = tk.Label(win, font=('', 20), anchor="center",highlightbackground='#3E4149')
label.pack(fill='x')
fr = tk.Frame(win)
fr.pack(expand = True)
fr.configure(background='#3E4149')

for y, cols in enumerate(BUTTONS):
    for x, n in enumerate(cols):
      btn = tk.Button(fr,text=n,font=('', 20),width=0,height=0,highlightbackground='#3E4149')
      btn.grid(row=y+1, column=x+1, sticky = 'nsew')
      btn.bind('<1>', make_click(n))


win.mainloop()