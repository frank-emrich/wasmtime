import gdb

initial_break = None

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
