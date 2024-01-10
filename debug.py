import gdb

initial_break = None

class WasmFxGdbEx(Exception):

  def __init__(self, msg):
    super().__init__(msg)



class Hunt (gdb.Command):
  """Greet the whole world."""

  def __init__ (self):
    super (Hunt, self).__init__ ("hunt", gdb.COMMAND_USER)

  def invoke (self, arg, from_tty):
    num = gdb.breakpoints()[-1].number
    print(num)

    gdb.execute("watch -location self.preview2_adapter.ptr.pointer.weak")
    gdb.execute("watch -location self.preview2_adapter.ptr.pointer.strong")
    weak_watch = gdb.breakpoints()[-2]
    strong_watch = gdb.breakpoints()[-1]
    weak_watch.ignore_count = 3
    strong_watch.ignore_count = 1

    print("self.preview2_adapter.ptr.pointer.strong is at following location:")
    gdb.execute("p &self.preview2_adapter.ptr.pointer.strong")
    print("Continue to go to offending write")

Hunt ()


def get_mappings():
  mappings = gdb.execute("info proc mappings",to_string=True)
  mappings = mappings.split("\n")

  result = []

  headings = ["Start Addr", "End Addr", "Size", "Offset", "Perms", "objfile"]
  past_header = False
  for m in mappings:
    if past_header:
      data = tuple(m.split())
      result.append(data)
    else:
      if all([ h in m for h in headings ]):
        past_header = True


  if not past_header:
    raise WasmFxGdbEx("Could not parse output of \"info proc mappings\"")
      
  print(result)


class WIP (gdb.Command):
  def __init__ (self):
    super (WIP, self).__init__ ("wip", gdb.COMMAND_USER)

  def invoke (self, arg, from_tty):
    get_mappings()

WIP ()
