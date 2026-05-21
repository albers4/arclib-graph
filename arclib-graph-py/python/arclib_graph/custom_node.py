import uuid


class CustomNode:
    def __init__(self):
        self._id = str(uuid.uuid4())

    @property
    def id(self) -> str:
        return self._id

    def __repr__(self):
        return f"<CustomNode id={self._id[:8]}>"
