# Jetbrains Lock Checker

When shutting down a Linux machine without exiting a Jetbrains editor correctly, residual `.lock` files can prevent the 
editor from starting again *(See this [StackOverflow post](https://stackoverflow.com/questions/77003028/error-while-opening-intellij-idea-due-to-an-already-running-process))*.

This program deletes those `.lock` files.

*You should not need this. Just shutdown your editors correctly*
