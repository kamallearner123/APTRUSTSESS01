
data = '''Play
1. Introduction
2min
Play
2. Exercises Overview
1min

Play
3. Idiomatic Code
6min
Play
4. Exercise - Idiomatic Code
9min
Play
5. Documentation
7min
Play
6. Exercise - Documentation
10min
Play
7. Publishing
6min
Play
8. Exercise - Publishing
1min
Play
9. Closures
3min
Play
10. Iterators
10min
Play
11. Exercise - Closures & Iterators
10min
Play
12. Common Traits
13min
Play
13. Exercise - Traits
10min
Play
14. Creating Errors
8min
Play
15. Handling Errors
12min
Play
16. Exercise - Errors
8min
Play
17. Unit Tests
11min
Play
18. Integration Tests
2min
Play
19. Benchmarks
6min
Play
20. Exercise - Testing
9min
Play
21. Logging
7min
Play
22. Exercise - Logging
7min
Play
23. Multithreading
9min
Play
24. Channels
11min
Play
25. Exercise - Threads & Channels
14min

Play
26. Project Overview
2min
Play
27. Tutorial: Configuration
3min
Play
28. Tutorial: Engine Initialization
1min
Play
29. Tutorial: Game State
3min
Play
30. Tutorial: Game Logic Function
2min
Play
31. Tutorial: Sprites
7min
Play
32. Tutorial: Colliders
10min
Play
33. Tutorial: Keyboard Input
3min
Play
34. Tutorial: Mouse Input
3min
Play
35. Tutorial: Text
5min
Play
36. Tutorial: Audio
3min
Play
37. Tutorial: Timer
3min
Play
38. Tutorial: Engine & Game Structs
6min
Play
39. Game Walkthrough: Common Setup
2min
Play
40. Game Walkthrough: Road Race
14min'''

import re
import os

progdir = os.getcwd()

def remove_dir(name):
    global cdir
    finalName = name.replace(" ","")
    finalName = finalName.replace(":","")
    os.rmdir(finalName)
    
def create_dir(name):
    currdir = os.getcwd()
    finalName = name.replace(" ","")
    os.mkdir(finalName)
    os.chdir(finalName)
    try:
        with open("Notes.txt", "w") as fd:
            fd.write(f"---- Notes on {name} ----")
        print(f"Empty file created")
    except Exception as e:
        print(f"Failed to create the file: {e}")
    os.chdir(currdir)
    
exp = re.compile(r"(\d{1,2}\. [\w :]+)")
filtered = re.findall(exp, data)
print(filtered)
for candidate in ["Kamal"]:
    os.mkdir(candidate)
    os.chdir(candidate)
    list(map(lambda d:create_dir(d), filtered))
    os.chdir(progdir)
#list(map(lambda d:create_dir(d), filtered))

