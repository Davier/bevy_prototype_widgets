
The goal of this builder pattern is to feel like "instantaneous commands". You can incrementally build the UI tree, and inspect and transform it before spawning it. It also enables reuse of UI sub-trees, saved as `Scene`s.

# Plan for styling

1. build widget tree with default style, setting style labels
2. set specific properties
3. apply style class from labels