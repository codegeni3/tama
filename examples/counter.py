from tama import storage

def increment():
    x = storage.get("counter")
    storage.set("counter", x + 1)
