import os
import sys
import time



def exe_rs():
    # print(os.getcwd())
    rs = None
    with open("last_rs_path", "r", encoding='utf-8') as f:
        rs = f.readline()
    
    # print(rs)
    if (rs is None):
        return
    
    print(os.system("rustc " + rs + " -o i9n0r3"))

    print(os.system("./i9n0r3"))
    



# argv1 = sys.argv[1]
# argv2 = sys.argv[2]


if __name__ == "__main__":
    exe_rs()
    # print(len(sys.argv))
    # print(sys.argv)
    # print(argv1)
    # print(argv2)
