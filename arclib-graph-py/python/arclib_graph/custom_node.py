import uuid


class CustomNode:
    def __init__(self):
        self._id = str(uuid.uuid4())

    @property
    def id(self) -> str:
        return self._id

    def compute(self):
        """
        Called by the graph engine during each time step.

        Override this method in your subclass to define what the node does.
        You can access internal state (self.state) or other attributes here.
        """
        raise NotImplementedError("Subclasses must implement compute()")

    def dependencies(self):
        """
        Called by the graph engine before execution.

        Override this method in your subclass to define what the node depends
        on. You can access internal state (self.state) or other attributes here.
        """
        raise NotImplementedError("Subclasses must implement dependencies()")

    def __repr__(self):
        return f"<CustomNode id={self._id}>"
