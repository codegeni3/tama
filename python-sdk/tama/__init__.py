class Storage:
    def get(self, key: str) -> int:
        """Get an integer value from contract storage for a given string key."""
        return 0

    def set(self, key: str, value: int) -> None:
        """Store an integer value in contract storage under a given string key."""
        pass

storage = Storage()

def contract(func):
    """Decorator to mark a function as a Soroban smart contract endpoint."""
    return func
